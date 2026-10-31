macro_rules! impl_physical {
    ( $($type:ident),+ ) => {
        $(
            impl Read for $type {
                fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                    let sector_size = self.geometry.bytes_per_sector;
                    if buf.len() < sector_size as usize {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("Could not read: buffer too small, must be at least {} bytes", sector_size)
                        ));
                    } else if (buf.len() % sector_size as usize) != 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!(
                                "Could not read: buffer size must be a multiple of {} but is {}",
                                sector_size, buf.len()
                            )
                        ));
                    }

                    let mut bytes_read = 0u32;
                    let res = unsafe {
                        fs::ReadFile(
                            self.handle,
                            buf.as_mut_ptr() as *mut c_void,
                            (buf.len() as u32 / sector_size) * sector_size,
                            &mut bytes_read,
                            null_mut()
                        )
                    };
                    if res == 0 {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Could not read: error code {:#08x}",
                                win32::last_error()
                            )
                        ))
                    } else {
                        Ok(bytes_read as usize)
                    }
                }
            }
            
            impl Write for $type {
                fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                    let sector_size = self.geometry.bytes_per_sector;
                    if buf.len() < sector_size as usize {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("Could not write: buffer too small, must be at least {} bytes", sector_size)
                        ));
                    } else if (buf.len() % sector_size as usize) != 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!(
                                "Could not write: buffer size must be a multiple of {} but is {}",
                                sector_size, buf.len()
                            )
                        ));
                    }

                    let mut bytes_written = 0u32;
                    let res = unsafe {
                        fs::WriteFile(
                            self.handle,
                            buf.as_ptr() as *mut c_void,
                            (buf.len() as u32 / sector_size) * sector_size,
                            &mut bytes_written,
                            null_mut()
                        )
                    };
                    if res == 0 {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Could not write: error code {:#08x}",
                                win32::last_error()
                            )
                        ))
                    } else {
                        Ok(bytes_written as usize)
                    }
                }
                
                fn flush(&mut self) -> std::io::Result<()> {
                    todo!();
                }
            }

            impl Seek for $type {
                fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
                    fn check_pos(s: &$type, p: u64) -> io::Result<()> {
                        if p > i64::MAX as u64 {
                            Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                format!("can only seek to pos <= {}", i64::MAX)
                            ))
                        } else if (p % s.geometry.bytes_per_sector as u64) != 0 {
                            Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                format!("can only seek to multiples of {}", s.geometry.bytes_per_sector)
                            ))
                        } else {
                            Ok(())
                        }
                    }

                    let mut dist = LARGE_INTEGER::default();
                    let mut new_pos = LARGE_INTEGER::default();

                    let res = match pos {
                        SeekFrom::Start(p) => {
                            check_pos(self, p)?;
                            unsafe {
                                *dist.QuadPart_mut() = p as i64;
                                fs::SetFilePointerEx(self.handle, dist, &mut new_pos, FILE_BEGIN)
                            }
                        },
                        SeekFrom::Current(p) => {
                            unsafe {
                                *dist.QuadPart_mut() = p;
                                fs::SetFilePointerEx(self.handle, dist, &mut new_pos, FILE_CURRENT)
                            }
                        },
                        SeekFrom::End(p) => {
                            // SetFilePointerEx seems to always fail when using FILE_END, therefore roll our own
                            let p = min(0, p); // limit p to non-positive values (for p > 0 we seek to the end)
                            return self.seek(SeekFrom::Start((self.size() as i128 + p as i128) as u64));
                        }
                    };

                    if res == 0 {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Could not seek: error code {} (did you respect sector boundaries?)", win32::last_error())
                        ))
                    } else {
                        Ok(unsafe { *new_pos.QuadPart() } as u64)
                    }
                }
            }

            impl Drop for $type {
                fn drop(&mut self) {
                    unsafe {
                        CloseHandle(self.handle);
                    }
                }
            }
        )+
    }
}

macro_rules! impl_buffered {
    ( $(($type:ident, $handle:ident)),+ ) => {
        $(
            impl $type {
                // updates the internal state so that current sector is the one with the given number
                // performs boundary checks (returns an error if sector_num too small, goes to last
                // sector if sector_num too big)
                fn go_to_sector(&mut self, sector_num: u64) -> io::Result<()> {
                    if sector_num == self.current_sector_num {
                        return Ok(());
                    }
                    let sector_num = min(sector_num, self.geometry.sectors());
                    let sector_size = self.$handle.geometry.bytes_per_sector as u64;
                    let sector_pos = sector_num * sector_size;
                    self.$handle.seek(SeekFrom::Start(sector_pos))?;

                    // last sector may not be of full length or length zero if max_pos % sector_size == 0
                    let max_pos = self.size();
                    let next_sector_len = min(sector_size, max_pos - sector_pos);
                    let mut sector = vec![0; next_sector_len as usize];
                    self.$handle.read_exact(&mut sector)?;

                    self.current_sector = Cursor::new(sector);
                    self.current_sector_num = sector_num;

                    Ok(())
                }
            }

            impl Read for $type {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                    if self.current_sector.position() == self.geometry.bytes_per_sector as u64 {
                        self.go_to_sector(self.current_sector_num + 1)?;
                    }

                    self.current_sector.read(buf)
                }
            }

            impl Seek for $type {
                fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
                    match pos {
                        SeekFrom::Start(p) => {
                            let sector = p / self.geometry.bytes_per_sector as u64;
                            self.go_to_sector(sector)?;
                            self.current_sector.seek(SeekFrom::Start(p % self.geometry.bytes_per_sector as u64))?;
                        },
                        SeekFrom::End(p) => {
                            let p = min(0, p); // limit p to non-positive values (for p > 0 we seek to the end)
                            let end = self.size() as i128;
                            let sector = (end + p as i128) / self.geometry.bytes_per_sector as i128;
                            self.go_to_sector(sector as u64)?;

                            let target_pos = (end + p as i128) - sector * self.geometry.bytes_per_sector as i128;
                            self.current_sector.seek(SeekFrom::Start(target_pos as u64))?;
                        },
                        SeekFrom::Current(p) => {
                            let current = (self.current_sector_num * self.geometry.bytes_per_sector as u64 + self.current_sector.position()) as i128;
                            let sector = (current + p as i128) / self.geometry.bytes_per_sector as i128;
                            self.go_to_sector(sector as u64)?;

                            let target_pos = (current + p as i128) - sector * self.geometry.bytes_per_sector as i128;
                            self.current_sector.seek(SeekFrom::Start(target_pos as u64))?;
                        }
                    }

                    Ok(self.current_sector_num * self.geometry.bytes_per_sector as u64 + self.current_sector.position())
                }
            }
        )+
    }
}
