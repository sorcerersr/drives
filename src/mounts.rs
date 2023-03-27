use crate::fs_wrap::read_lines;

#[derive(Debug)]
pub struct Mount {
    device: String,
    mountpoint: String,
    filesystem: String,
}

pub fn read_mountpoints() {
    let mut mounts = vec![];
    if let Ok(lines) = read_lines("/proc/mounts") {
        for line in lines {
            if let Ok(mount) = line {
                let numerates = mount.split_whitespace().enumerate();
                let item: Vec<&str> = numerates.map(|(_, e)| e).collect();
                mounts.push(Mount {
                    device: item[0].to_owned(),
                    mountpoint: item[1].to_owned(),
                    filesystem: item[2].to_owned(),
                });
            }
        }
    }
    for mount in mounts {
        println!("{:?}", mount);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_mountpoints() {
        read_mountpoints()
    }
}
