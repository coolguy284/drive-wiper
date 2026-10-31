# windows-drives
[![](https://ico.y.gy/crates/v/windows-drives)](https://ico.y.gy/crates/v/windows-drives)

This crate provides safe wrappers around the unsafe Win32 API provided by `winapi` for raw disk access.

Currently accessing complete disks (`\\.\PhysicalDriveX`) and disk partitions (`\\?\HarddiskVolumeX`) is supported.

A good program to view your computer's disk mapping is [SIV](http://rh-software.com/) (under "Volumes -> Disk Mapping").
