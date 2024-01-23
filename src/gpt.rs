use crate::Device;
#[cfg(feature = "gpt")]
use gpt;

#[cfg(not(test))]
const DEV_DIR: &str = "/dev/";

#[cfg(test)]
const DEV_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources", "/test");

/// Enumeration for holding the gpt UUID or a reason why it is not available
#[derive(Debug)]
pub enum GptUUID {
    /// an io error happened when opening the device for read access
    IoError(std::io::Error),
    /// the UUID from the partition table (gpt) as a hyphenated string
    UUID(String),
    /// the feature "gpt" was not enabled
    FeatureNotEnabled,
    /// reading the gpt was successful, but no header was found
    NotAvailable,
}

// when the feature "gpt" is not enabled this function is used
// to set the GptUUID::FeatureNotEnabled value
#[cfg(not(feature = "gpt"))]
pub fn enrich_with_gpt_uuid(mut device: Device) -> Device {
    device.uuid = GptUUID::FeatureNotEnabled;
    device
}

// When the feature "gpt" is enabled then this function will actually read the
// partition table (gpt) to get the UUID for the device and the partitions
#[cfg(feature = "gpt")]
pub fn enrich_with_gpt_uuid(mut device: Device) -> Device {

    let diskpath = std::path::Path::new(DEV_DIR).join(device.name.to_string());
    let cfg = gpt::GptConfig::new().writable(false);
    match cfg.open(diskpath) {
        Err(error) => device.uuid = GptUUID::IoError(error),
        Ok(disk) => {
            match disk.primary_header() {
            None => device.uuid = GptUUID::NotAvailable,
            Some(disk_header) => {
                device.uuid = GptUUID::UUID(disk_header.disk_guid.as_hyphenated().to_string());
                }

            };
            for partition in device.partitions.iter_mut() {
                match disk.partitions().get(&partition.number){
                    Some(gpt_partition) => partition.part_uuid = GptUUID::UUID(gpt_partition.part_guid.as_hyphenated().to_string()),
                    None => partition.part_uuid = GptUUID::NotAvailable,
                }
            }
        },
    };

    device
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(feature = "gpt")]
    #[test]
    fn test_enrich_with_gpt_uuid() {
        use crate::Size;

        let mut device = Device {
            name: "gptdisk.img".to_string(),
            partitions: Vec::new(),
            is_removable: false,
            model: None,
            serial: None,
            size: Size::new(42),
            uuid: GptUUID::NotAvailable,
        };
        device = enrich_with_gpt_uuid(device);

        match device.uuid {
            GptUUID::UUID(uuid) => assert_eq!("f0ce7b2c-74af-47e4-8141-b2fe24ac20cc", uuid),
            _ => panic!("No UUID"),
        }
    }
}
