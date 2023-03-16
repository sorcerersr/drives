//!
//! `Drives` is a library to get information about mountable/mounted drives
//! on a Linux system.
//!
//! Uses `/sys/block` to retreive information.
//!

use anyhow::{anyhow, Context, Result};

use std::fs::DirEntry;

use fs_wrap::read_file_to_string;

mod fs_wrap;

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

fn read_bool_file(path: &str) -> Result<bool> {
    let content =
        read_file_to_string(path).with_context(|| format!("Failed to read file from {}", path))?;
    Ok("1".eq(&content))
}

fn name_from_direntry(entry: &DirEntry) -> Result<String> {
    if let Ok(result) = entry.file_name().into_string() {
        return Ok(result);
    } else {
        return Err(anyhow!("Failed to convert OsString to String"));
    }
}

fn build_path(dir_entry: &DirEntry, path_to_add: &str) -> Result<String> {
    let mut path = if let Some(path) = dir_entry.path().to_str() {
        path.to_string()
    } else {
        return Err(anyhow!("Failed to append '{}' to path", path_to_add));
    };
    path.push_str(path_to_add);
    Ok(path)
}

fn find_partitions(dir_entry: &DirEntry) -> Result<Vec<Partition>> {
    let mut partitions = vec![];
    let base_dir_name = name_from_direntry(dir_entry)?;

    for entry in fs_wrap::read_dir(dir_entry.path())
        .with_context(|| format!("Failed to read dir {:#?}", dir_entry.path()))?
    {
        let entry = entry.with_context(|| "Failed to access dir entry")?;
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                let dir_name = name_from_direntry(&entry)?;
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

/// Reads /sys/block and its sub-directories to determine and return drives as a list of
/// devices with partitions
pub fn get_drives() -> Result<Vec<Device>> {
    let mut devices = vec![];
    for entry in fs_wrap::read_dir("/sys/block").with_context(|| "Failed to access /sys/block")? {
        let entry = entry.with_context(|| "Failed to access dir entry")?;

        let device_name = name_from_direntry(&entry)?;

        let removable_path = build_path(&entry, "/removable")?;
        let removable = read_bool_file(&removable_path)?;

        let partitions = find_partitions(&entry)?;

        let device = Device {
            name: device_name.clone(),
            partitions,
            is_removable: removable,
        };
        devices.push(device);
    }
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let drives = get_drives().unwrap();
        drives.iter().for_each(|drive| {
            println!("{:?}", drive);
        })
    }
}
