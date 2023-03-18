//!
//! `Drives` is a library to get information about mountable/mounted drives
//! on a Linux system.
//!
//! Uses `/sys/block` to retreive information.
//!

use anyhow::{anyhow, Context, Result};

mod fs_wrap;

use std::fs::DirEntry;

/// A block device
#[derive(Debug)]
pub struct Device {
    /// The name of the block device
    pub name: String,
    /// list of partitions
    pub partitions: Vec<Partition>,
    /// is it a fixed device or a removable one like a flash drive or sd card
    pub is_removable: bool,
}

/// partition of a device
#[derive(Debug)]
pub struct Partition {
    /// the name of the partitions
    pub name: String,
}

struct Drives {
    base_path: String,
}

impl Drives {
    fn find_partitions(&self, dir_entry: &DirEntry) -> Result<Vec<Partition>> {
        let mut partitions = vec![];
        let base_dir_name = fs_wrap::name_from_direntry(dir_entry)?;
        let dir_entry_path = dir_entry
            .path()
            .to_str()
            .with_context(|| format!("failed calling to_str on path {:?}", dir_entry.path()))?
            .to_owned();
        for entry in fs_wrap::read_dir(&dir_entry_path)
            .with_context(|| format!("Failed to read dir {:#?}", dir_entry.path()))?
        {
            let entry = entry.with_context(|| "Failed to access dir entry")?;
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let dir_name = fs_wrap::name_from_direntry(&entry)?;
                    if dir_name.starts_with(&base_dir_name) {
                        partitions.push(Partition { name: dir_name });
                    }
                }
            } else {
                return Err(anyhow!("Couldn't get file type for {:?}", entry.path()));
            }
        }
        Ok(partitions)
    }

    fn get_drives(&self) -> Result<Vec<Device>> {
        let mut devices = vec![];
        for entry in
            fs_wrap::read_dir(&self.base_path).with_context(|| "Failed to access /sys/block")?
        {
            let entry = entry.with_context(|| "Failed to access dir entry")?;

            let device_name = fs_wrap::name_from_direntry(&entry)?;

            let removable_path = fs_wrap::build_path(&entry, "/removable")?;
            let removable = fs_wrap::read_bool_file(&removable_path)?;

            let partitions = self.find_partitions(&entry)?;

            let device = Device {
                name: device_name.clone(),
                partitions,
                is_removable: removable,
            };
            devices.push(device);
        }
        Ok(devices)
    }

    fn new() -> Drives {
        Drives {
            base_path: "/sys/block".to_owned(),
        }
    }
}

/// Reads /sys/block and its sub-directories to determine and return drives as a list of
/// devices with partitions
pub fn get_devices() -> Result<Vec<Device>> {
    let drives = Drives::new();
    drives.get_drives()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_drives() {
        let drives = get_devices().unwrap();
        drives.iter().for_each(|drive| {
            println!("{:?}", drive);
        })
    }
}
