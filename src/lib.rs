use std::fs;

#[derive(Debug)]
pub enum Error {
    SysDirAccess,
    SysDirEntry,
    SysDirEntryName,
}

#[derive(Debug)]
pub struct Drive {
    pub name: String,
}

pub fn get_drives() -> Result<Vec<Drive>, Error> {
    let mut drives = vec![];
    for entry in fs::read_dir("/sys/block").map_err(|_err| Error::SysDirAccess)? {
        let entry = entry.map_err(|_err| Error::SysDirEntry)?;
        let drive = Drive {
            name: entry
                .file_name()
                .into_string()
                .map_err(|_err| Error::SysDirEntryName)?,
        };
        drives.push(drive);
    }
    Ok(drives)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let drives = get_drives().unwrap();
        drives.iter().for_each(|drive| {
            println!("{}", drive.name);
        })
    }
}
