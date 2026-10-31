/*!
This crate provides safe wrappers around the unsafe Win32 API provided by `winapi` for raw disk access.

Currently accessing complete disks (`\\.\PhysicalDriveX`) and disk partitions (`\\?\HarddiskVolumeX`) is supported.

A good program to view your computer's disk mapping is [SIV](http://rh-software.com/) (under "Volumes -> Disk Mapping").
*/

#[macro_use]
pub(crate) mod macros;

/// The core module of the crate, providing the data structures representing physical drives.
pub mod drive;

/// A module providing helper functions to interact with the Win32 API.
pub mod win32;

pub use drive::*;
