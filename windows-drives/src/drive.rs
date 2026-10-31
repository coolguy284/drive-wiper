use crate::win32;
use derive_try_from_primitive::TryFromPrimitive;
use std::{
    cmp::min,
    convert::TryFrom,
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    mem::size_of,
    ptr::null_mut,
};
use winapi::{
    ctypes::c_void,
    shared::ntdef::LARGE_INTEGER,
    um::{
        fileapi::{self as fs, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        ioapiset::DeviceIoControl,
        winbase::{FILE_BEGIN, FILE_CURRENT, FILE_FLAG_NO_BUFFERING},
        winioctl::{
            DISK_GEOMETRY, IOCTL_DISK_GET_DRIVE_GEOMETRY, IOCTL_DISK_GET_PARTITION_INFO_EX,
            PARTITION_INFORMATION_EX, PARTITION_STYLE_GPT, PARTITION_STYLE_MBR,
            PARTITION_STYLE_RAW,
        },
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
    },
};

/// Gives direct access to a physical drive.
///
/// NOTE: because of Win32 restrictions, you must `read()` whole sectors and must only `seek()` to sector
/// boundaries. If you need random access, use [`BufferedPhysicalDrive`].
#[derive(Debug)]
pub struct PhysicalDrive {
    handle: HANDLE,
    /// The [`DiskGeometry`] of the drive.
    pub geometry: DiskGeometry,
}

/// Gives buffered direct access to a physical drive to enable random access.
///
/// Also see [`PhysicalDrive`].
#[derive(Debug)]
pub struct BufferedPhysicalDrive {
    drive: PhysicalDrive,
    current_sector: Cursor<Vec<u8>>,
    current_sector_num: u64,
    /// The [`DiskGeometry`] of the drive.
    pub geometry: DiskGeometry,
}

/// Gives direct access to a harddisk volume.
///
/// NOTE: because of Win32 restrictions, you must `read()` whole sectors and must only `seek()` to sector
/// boundaries. If you need random access, use [`BufferedHarddiskVolume`].
#[derive(Debug)]
pub struct HarddiskVolume {
    handle: HANDLE,
    /// The [`DiskGeometry`] of the drive this volume is located on.
    pub geometry: DiskGeometry,
    /// The [`PartitionInfo`] of the volume.
    pub partition_info: PartitionInfo,
}

/// Gives buffered direct access to a harddisk volume to enable random access.
///
/// Also see [`HarddiskVolume`].
#[derive(Debug)]
pub struct BufferedHarddiskVolume {
    volume: HarddiskVolume,
    current_sector: Cursor<Vec<u8>>,
    current_sector_num: u64,
    /// The [`DiskGeometry`] of the drive this volume is located on.
    pub geometry: DiskGeometry,
    /// The [`PartitionInfo`] of the volume.
    pub partition_info: PartitionInfo,
}

/// Represents the geometry of a disk or physical drive.
#[derive(Copy, Clone, Debug)]
pub struct DiskGeometry {
    pub cylinders: i64,
    pub media_type: u32,
    pub tracks_per_cylinder: u32,
    pub sectors_per_track: u32,
    pub bytes_per_sector: u32,
}

/// The style of a partition.
#[repr(u32)]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
pub enum PartitionStyle {
    Gpt = PARTITION_STYLE_GPT,
    Mbr = PARTITION_STYLE_MBR,
    Raw = PARTITION_STYLE_RAW,
}

/// Information about a partition.
#[derive(Copy, Clone, Debug)]
pub struct PartitionInfo {
    pub partition_style: PartitionStyle,
    pub starting_offset: u64,
    pub partition_length: u64,
    pub partition_number: u32,
}

impl PhysicalDrive {
    /// Opens the physical drive with the given number.
    ///
    /// NOTE: this requires administrator privileges. Drive numbers start at 0.
    ///
    /// Fails if an invalid drive number is given, the user has insufficient privileges
    /// or an error occures while opening the drive.
    /// If an error occurs, an error message containing the error number is returned, see
    /// [here](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes) for a list.
    pub fn open(drive_num: u8) -> Result<Self, String> {
        let path = format!("\\\\.\\PhysicalDrive{}", drive_num);
        let handle = open_handle(&path)?;
        Ok(PhysicalDrive {
            handle,
            geometry: geometry(&handle)?,
        })
    }

    /// The size of the drive in bytes.
    pub fn size(&self) -> u64 {
        self.geometry.size()
    }
}

impl HarddiskVolume {
    /// Opens the harddisk volume with the given number.
    ///
    /// NOTE: this requires administrator privileges. Volume numbers start at 1.
    ///
    /// Fails if an invalid harddisk volume number is given, the user has insufficient privileges
    /// or an error occures while opening the volume.
    /// If an error occurs, an error message containing the error number is returned, see
    /// [here](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes) for a list.
    pub fn open(volume_num: u8) -> Result<Self, String> {
        let path = format!("\\\\?\\HarddiskVolume{}", volume_num);
        let handle = open_handle(&path)?;
        Ok(HarddiskVolume {
            handle,
            geometry: geometry(&handle)?,
            partition_info: partition_info(&handle)?,
        })
    }

    /// The size of the volume in bytes.
    pub fn size(&self) -> u64 {
        self.partition_info.partition_length
    }
}

impl_physical!(PhysicalDrive, HarddiskVolume);

impl BufferedPhysicalDrive {
    /// Opens the physical drive with the given number.
    ///
    /// NOTE: this requires administrator privileges. Drive numbers start at 0.
    ///
    /// Fails if an invalid drive number is given, the user has insufficient privileges
    /// or an error occures while opening the drive.
    /// If an error occurs, an error message containing the error number is returned, see
    /// [here](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes) for a list.
    pub fn open(drive_num: u8) -> Result<Self, String> {
        let mut drive = PhysicalDrive::open(drive_num)?;
        let mut sector = vec![0; drive.geometry.bytes_per_sector as usize];
        match drive.read(&mut sector) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        }
        let geo = drive.geometry;
        Ok(BufferedPhysicalDrive {
            drive,
            current_sector: Cursor::new(sector),
            current_sector_num: 0,
            geometry: geo,
        })
    }

    /// The size of the drive in bytes.
    pub fn size(&self) -> u64 {
        self.geometry.size()
    }
}

impl BufferedHarddiskVolume {
    /// Opens the harddisk volume with the given number.
    ///
    /// NOTE: this requires administrator privileges. Volume numbers start at 1.
    ///
    /// Fails if an invalid harddisk volume number is given, the user has insufficient privileges
    /// or an error occures while opening the colume.
    /// If an error occurs, an error message containing the error number is returned, see
    /// [here](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes) for a list.
    pub fn open(volume_num: u8) -> Result<Self, String> {
        let mut volume = HarddiskVolume::open(volume_num)?;
        let mut sector = vec![0; volume.geometry.bytes_per_sector as usize];
        match volume.read(&mut sector) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        }
        let geo = volume.geometry;
        let info = volume.partition_info;
        Ok(BufferedHarddiskVolume {
            volume,
            current_sector: Cursor::new(sector),
            current_sector_num: 0,
            geometry: geo,
            partition_info: info,
        })
    }

    /// The size of the volume in bytes.
    pub fn size(&self) -> u64 {
        self.partition_info.partition_length
    }
}

impl_buffered!(
    (BufferedPhysicalDrive, drive),
    (BufferedHarddiskVolume, volume)
);

impl DiskGeometry {
    /// Returns the size of the disk in bytes.
    pub fn size(&self) -> u64 {
        self.sectors() * self.bytes_per_sector as u64
    }

    /// Returns the number of sectors of the disk.
    pub fn sectors(&self) -> u64 {
        self.cylinders as u64 * self.tracks_per_cylinder as u64 * self.sectors_per_track as u64
    }
}

impl From<DISK_GEOMETRY> for DiskGeometry {
    fn from(geo: DISK_GEOMETRY) -> Self {
        DiskGeometry {
            cylinders: unsafe { *geo.Cylinders.QuadPart() },
            media_type: geo.MediaType,
            tracks_per_cylinder: geo.TracksPerCylinder,
            sectors_per_track: geo.SectorsPerTrack,
            bytes_per_sector: geo.BytesPerSector,
        }
    }
}

impl From<PARTITION_INFORMATION_EX> for PartitionInfo {
    fn from(info: PARTITION_INFORMATION_EX) -> Self {
        PartitionInfo {
            partition_style: PartitionStyle::try_from(info.PartitionStyle).unwrap(),
            starting_offset: unsafe { *info.StartingOffset.QuadPart() } as _,
            partition_length: unsafe { *info.PartitionLength.QuadPart() } as _,
            partition_number: info.PartitionNumber,
        }
    }
}

fn open_handle(path: &str) -> Result<HANDLE, String> {
    let path = win32::win32_string(&path);
    let handle = unsafe {
        fs::CreateFileW(
            path.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            FILE_FLAG_NO_BUFFERING,
            null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        let err = win32::last_error();
        Err(match err {
            2 => "could not open handle because the device was not found".to_string(),
            5 => "could not open handle because access was denied - do you have administrator privileges?".to_string(),
            _ => format!("got invalid handle: error code {:#08x}", err)
        })
    } else {
        Ok(handle)
    }
}

// Gets the disk geometry via the Win32 API.
// If an error occurs, an error message containing the error number is returned.
fn geometry(drive: &HANDLE) -> Result<DiskGeometry, String> {
    let mut geo = Default::default();
    let mut bytes_returned = 0u32;
    let geo_ptr: *mut DISK_GEOMETRY = &mut geo;
    let r = unsafe {
        DeviceIoControl(
            *drive,
            IOCTL_DISK_GET_DRIVE_GEOMETRY,
            null_mut(),
            0,
            geo_ptr as *mut c_void,
            size_of::<DISK_GEOMETRY>() as u32,
            &mut bytes_returned,
            null_mut(),
        )
    };
    if r == 0 {
        Err(format!(
            "could not get geometry: error code {:#08x}",
            win32::last_error()
        ))
    } else {
        Ok(DiskGeometry::from(geo))
    }
}

// Gets the disk geometry via the Win32 API.
// If an error occurs, an error message containing the error number is returned.
fn partition_info(partition: &HANDLE) -> Result<PartitionInfo, String> {
    let mut info = Default::default();
    let mut bytes_returned = 0u32;
    let info_ptr: *mut PARTITION_INFORMATION_EX = &mut info;
    let r = unsafe {
        DeviceIoControl(
            *partition,
            IOCTL_DISK_GET_PARTITION_INFO_EX,
            null_mut(),
            0,
            info_ptr as *mut c_void,
            size_of::<PARTITION_INFORMATION_EX>() as u32,
            &mut bytes_returned,
            null_mut(),
        )
    };
    if r == 0 {
        Err(format!(
            "could not get partition info: error code {:#08x}",
            win32::last_error()
        ))
    } else {
        Ok(PartitionInfo::from(info))
    }
}
