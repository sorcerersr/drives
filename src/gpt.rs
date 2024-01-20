use crate::Device;
#[cfg(feature = "gpt")]
use gpt;


#[cfg(not(test))]
const DEV_DIR:&str = "/dev/";

#[cfg(test)]
const DEV_DIR:&str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources", "/test");

#[derive(Debug)]
pub enum GptUUID {
    IoError(std::io::Error),
    UUID(String),
    FeatureNotEnabled,
    NotAvailable,
}

#[cfg(not(feature = "gpt"))]
pub fn enrich_with_gpt_uuid(mut device: Device) -> Device {
    
    device.uuid = GptUUID::FeatureNotEnabled;
    device
}

#[cfg(feature = "gpt")]
pub fn enrich_with_gpt_uuid(mut device: Device) -> Device {
    
    let diskpath = std::path::Path::new(DEV_DIR).join(device.name.to_string());
    let cfg = gpt::GptConfig::new().writable(false);
    match cfg.open(diskpath) {
        Err(error) => device.uuid = GptUUID::IoError(error),
        Ok(disk) => {
            match disk.primary_header() {
                None => device.uuid = GptUUID::NotAvailable,
                Some(disk_header) => device.uuid = GptUUID::UUID(disk_header.disk_guid.as_hyphenated().to_string())
            }        
        }
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