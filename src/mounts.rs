use crate::{error::DrivesError, fs_wrap::read_lines};

/// Informations about the mount point of a drives patition
#[derive(Debug, Clone)]
pub struct Mount {
    /// the partitions linux device name
    pub device: String,
    /// the path of the mount point
    pub mountpoint: String,
    /// the filesystem used to mount the partition
    pub filesystem: String,
}

pub struct Mounts {
    mount_file_path: String,
}

impl Mounts {
    pub fn read_mountpoints(&self) -> Result<Vec<Mount>, DrivesError> {
        let mut mounts = vec![];
        if let Ok(lines) = read_lines(&self.mount_file_path) {
            lines.for_each(|line| {
                if let Ok(mount) = line {
                    let numerates = mount.split_whitespace().enumerate();
                    let item: Vec<&str> = numerates.map(|(_, e)| e).collect();
                    mounts.push(Mount {
                        device: item[0].to_owned(),
                        mountpoint: item[1].to_owned(),
                        filesystem: item[2].to_owned(),
                    });
                }
            });
        } else {
            return Err(DrivesError::ReadingMountsFailed);
        }

        Ok(mounts)
    }

    pub fn new() -> Mounts {
        Mounts {
            mount_file_path: "/proc/mounts".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::Write;

    use tempfile::NamedTempFile;

    #[test]
    fn test_read_mounts() {
        let mut test_file = NamedTempFile::new().unwrap();
        let content = concat!(
            "/dev/sda1 / ext4 rw,relatime 0 0\n",
            "/dev/sda2 /home ext4 rw,relatime 0 0"
        );

        test_file.write_all(content.as_bytes()).unwrap();

        let mounts = Mounts {
            mount_file_path: test_file.path().to_str().unwrap().to_owned(),
        };
        let result = mounts.read_mountpoints().unwrap();
        let sda1 = result
            .iter()
            .find(|mount| mount.device == "/dev/sda1")
            .unwrap();
        assert_eq!(sda1.device, "/dev/sda1");
        assert_eq!(sda1.filesystem, "ext4");
        assert_eq!(sda1.mountpoint, "/");

        let sda2 = result
            .iter()
            .find(|mount| mount.device == "/dev/sda2")
            .unwrap();
        assert_eq!(sda2.device, "/dev/sda2");
        assert_eq!(sda2.filesystem, "ext4");
        assert_eq!(sda2.mountpoint, "/home");
    }
}
