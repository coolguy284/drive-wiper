use std::sync::LazyLock;

use regex::Regex;

pub(crate) const STATUS_VERSION: u64 = 2;
pub(crate) const STATUS_JSON_INDENT: u16 = 2;
pub(crate) const AVERAGE_SPEED_SAMPLE_COUNT: usize = 500;

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton/27826181#27826181
#[cfg(target_os = "windows")]
pub(crate) static PHYSICAL_DRIVE_WIN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\\.\\PhysicalDrive(\d+)").unwrap());

#[cfg(target_os = "windows")]
pub(crate) static DRIVE_PARTITION_WIN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\\?\\HarddiskVolume(\d+)").unwrap());
