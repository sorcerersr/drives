//!
//! `Drives` is a library to get information about mountable/mounted drives
//! on a Linux system.
//!
//! Uses `/sys/block` to retreive information.
//!

use anyhow::{anyhow, Context, Result};
use fs_wrap::build_path;
use mounts::{Mount, Mounts};

mod fs_wrap;
mod mounts;

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
    /// the model string - most common a combination of vendor name and model identifier
    /// will be None for virtual devices
    pub model: Option<String>,
    /// the hardware serial string
    pub serial: Option<String>,
    pub size: u64,
}

/// partition of a device
#[derive(Debug)]
pub struct Partition {
    /// the name of the partitions
    pub name: String,
    /// size of the partition on 512 byte blocks
    pub size: u64,
    /// the mountpoint if mounted
    pub mountpoint: Option<String>,
}

struct Drives {
    base_path: String,
    mounts: Mounts,
}

impl Drives {
    fn find_partitions(&self, dir_entry: &DirEntry) -> Result<Vec<Partition>> {
        let mount_points = self.mounts.read_mountpoints()?;
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
                        let size = fs_wrap::read_file_to_u64(&build_path(&entry, "/size")?)?;
                        let mount = self.find_mountpoint_for_partition(&mount_points, &dir_name)?;
                        partitions.push(Partition {
                            name: dir_name,
                            size,
                            mountpoint: mount,
                        });
                    }
                }
            } else {
                return Err(anyhow!("Couldn't get file type for {:?}", entry.path()));
            }
        }
        Ok(partitions)
    }

    fn find_mountpoint_for_partition(
        &self,
        mounts: &[Mount],
        partition_name: &str,
    ) -> Result<Option<String>> {
        let found_mount = mounts
            .iter()
            .find(|mount| mount.device.contains(partition_name));
        if let Some(mount) = found_mount {
            return Ok(Some(mount.mountpoint.to_owned()));
        }
        Ok(None)
    }

    fn read_model_and_serial_if_available(
        &self,
        dir_entry: &DirEntry,
    ) -> (Option<String>, Option<String>) {
        let device_subdir_path = dir_entry.path().join("device");

        if !device_subdir_path.exists() {
            return (None, None);
        }
        let model_file_path = device_subdir_path.join("model");
        let serial_file_path = device_subdir_path.join("serial");
        let model = fs_wrap::read_file_to_string(model_file_path.as_path()).ok();
        let serial = fs_wrap::read_file_to_string(serial_file_path.as_path()).ok();

        (model, serial)
    }

    fn get_devices(&self) -> Result<Vec<Device>> {
        let mut devices = vec![];
        for entry in
            fs_wrap::read_dir(&self.base_path).with_context(|| "Failed to access /sys/block")?
        {
            let entry = entry.with_context(|| "Failed to access dir entry")?;

            let device_name = fs_wrap::name_from_direntry(&entry)?;

            let removable_path = fs_wrap::build_path(&entry, "/removable")?;
            let removable = fs_wrap::read_bool_file(&removable_path)?;

            let partitions = self.find_partitions(&entry)?;

            let model_and_serial = self.read_model_and_serial_if_available(&entry);
            let size = fs_wrap::read_file_to_u64(&build_path(&entry, "/size")?)?;

            let device = Device {
                name: device_name.clone(),
                partitions,
                is_removable: removable,
                model: model_and_serial.0,
                serial: model_and_serial.1,
                size,
            };
            devices.push(device);
        }
        Ok(devices)
    }

    fn new() -> Drives {
        Drives {
            base_path: "/sys/block".to_owned(),
            mounts: Mounts::new(),
        }
    }
}

/// Reads /sys/block and its sub-directories to determine and return drives as a list of
/// devices with partitions
pub fn get_devices() -> Result<Vec<Device>> {
    let drives = Drives::new();
    drives.get_devices()
}

#[cfg(test)]
mod tests {

    use tempfile::tempdir;

    use super::*;

    use std::{fs, io::Write};

    #[test]
    fn test_drives() {
        // a temp dir to represent /sys/block
        let temp_dir = tempdir().unwrap();

        // a subdir for a device (in this example an nvme drive=
        let next_dir_path = temp_dir.path().join("nvme0n1");
        fs::create_dir(&next_dir_path).unwrap();

        // create a "removable" flag file for the device
        let mut removable_file = fs::File::create(next_dir_path.join("removable")).unwrap();
        removable_file.write_all("0".as_bytes()).unwrap();

        let mut size_file = fs::File::create(next_dir_path.join("size")).unwrap();
        size_file.write_all("1000215216".as_bytes()).unwrap();

        // now create two partitions that are represented by subfolders
        let part_one_dir_path = next_dir_path.join("nvme0n1p1");
        fs::create_dir(&part_one_dir_path).unwrap();
        size_file = fs::File::create(part_one_dir_path.as_path().join("size")).unwrap();
        size_file.write_all("1050624".as_bytes()).unwrap();

        let part_two_dir_path = next_dir_path.join("nvme0n1p2");
        fs::create_dir(&part_two_dir_path).unwrap();
        size_file = fs::File::create(part_two_dir_path.as_path().join("size")).unwrap();
        size_file.write_all("999162511".as_bytes()).unwrap();
        // and create a third dir that isn't following the partition name schema
        // and should therefor not be identified as a partition
        let power_dir_path = next_dir_path.join("power");
        fs::create_dir(power_dir_path).unwrap();

        // execute
        let drives = Drives {
            base_path: temp_dir.path().to_str().unwrap().to_owned(),
            mounts: Mounts::new(),
        };
        let devices = drives.get_devices().unwrap();

        // now verify the results
        assert_eq!(1, devices.len());
        let device = devices.get(0).unwrap();
        assert_eq!("nvme0n1", device.name);
        assert!(!device.is_removable);
        assert_eq!(2, device.partitions.len());
        let part1 = device
            .partitions
            .iter()
            .find(|part| part.name.eq("nvme0n1p1"));
        assert!(part1.is_some());
        let part2 = device
            .partitions
            .iter()
            .find(|part| part.name.eq("nvme0n1p2"));
        assert!(part2.is_some());
    }
}
