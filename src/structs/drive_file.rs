use std::error::Error;
use std::fs::{File, metadata};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

// https://stackoverflow.com/questions/51821451/how-to-use-a-crate-only-for-a-given-platform/51821632#51821632
#[cfg(target_os = "windows")]
use windows_drives::{HarddiskVolume, PhysicalDrive};

#[cfg(target_os = "windows")]
use crate::consts::{DRIVE_PARTITION_WIN_REGEX, PHYSICAL_DRIVE_WIN_REGEX};

#[cfg(target_os = "windows")]
enum DriveFileBacking {
  File(File),
  Disk(PhysicalDrive),
  Partition(HarddiskVolume),
}

#[cfg(not(target_os = "windows"))]
enum DriveFileBacking {
  File(File),
}

pub(crate) struct DriveFile {
  backing: DriveFileBacking,
}

impl DriveFile {
  pub(crate) fn open<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
      let path_string = path.as_ref().to_string_lossy();
      
      if let Some(regex_captures) = PHYSICAL_DRIVE_WIN_REGEX.captures(&path_string) {
        if let Some(regex_match) = regex_captures.get(1) {
          let drive_index = regex_match.as_str().parse::<u8>()?;
          
          return Ok(DriveFile {
            backing: DriveFileBacking::Disk(PhysicalDrive::open(drive_index, true, true)?),
          })
        }
      }
      
      if let Some(regex_captures) = DRIVE_PARTITION_WIN_REGEX.captures(&path_string) {
        if let Some(regex_match) = regex_captures.get(1) {
          let partition_index = regex_match.as_str().parse::<u8>()?;
          
          return Ok(DriveFile {
            backing: DriveFileBacking::Partition(HarddiskVolume::open(partition_index, true, true)?),
          })
        }
      }
    }
    
    Ok(DriveFile {
      backing: DriveFileBacking::File(
        File::options()
          .read(true)
          .write(true)
          .open(path)?
      ),
    })
  }
  
  pub(crate) fn get_drive_size<P: AsRef<Path>>(path: P) -> Result<u64, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
      let path_string = path.as_ref().to_string_lossy();
      
      if let Some(_regex_match) = PHYSICAL_DRIVE_WIN_REGEX.find(&path_string) {
        Self::open(path)?.size()
      } else if let Some(_regex_match) = DRIVE_PARTITION_WIN_REGEX.find(&path_string) {
        Self::open(path)?.size()
      } else {
        Ok(metadata(path)?.len())
      }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
      Ok(metadata(path)?.len())
    }
  }
  
  pub(crate) fn size(&self) -> Result<u64, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    match &self.backing {
      DriveFileBacking::File(file) => Ok(file.metadata()?.len()),
      DriveFileBacking::Disk(disk) => Ok(disk.size()),
      DriveFileBacking::Partition(partition) => Ok(partition.size()),
    }
    
    #[cfg(not(target_os = "windows"))]
    match &self.backing {
      DriveFileBacking::File(file) => Ok(file.metadata()?.len()),
    }
  }
  
  pub(crate) fn seek(&mut self, pos: SeekFrom) -> Result<u64, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.seek(pos)?),
      DriveFileBacking::Disk(ref mut disk) => Ok(disk.seek(pos)?),
      DriveFileBacking::Partition(ref mut partition) => Ok(partition.seek(pos)?),
    }
    
    #[cfg(not(target_os = "windows"))]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.seek(pos)?),
    }
  }
  
  pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.read_exact(buf)?),
      DriveFileBacking::Disk(ref mut disk) => Ok(disk.read_exact(buf)?),
      DriveFileBacking::Partition(ref mut partition) => Ok(partition.read_exact(buf)?),
    }
    
    #[cfg(not(target_os = "windows"))]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.read_exact(buf)?),
    }
  }
  
  pub(crate) fn write_all(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.write_all(buf)?),
      DriveFileBacking::Disk(ref mut disk) => Ok(disk.write_all(buf)?),
      DriveFileBacking::Partition(ref mut partition) => Ok(partition.write_all(buf)?),
    }
    
    #[cfg(not(target_os = "windows"))]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.write_all(buf)?),
    }
  }
  
  pub(crate) fn sync_data(&mut self) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.sync_data()?),
      DriveFileBacking::Disk(ref mut disk) => Ok(disk.sync_data()?),
      DriveFileBacking::Partition(ref mut partition) => Ok(partition.sync_data()?),
    }
    
    #[cfg(not(target_os = "windows"))]
    match self.backing {
      DriveFileBacking::File(ref mut file) => Ok(file.sync_data()?),
    }
  }
}
