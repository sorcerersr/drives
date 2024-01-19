use crate::Device;
#[cfg(feature = "gpt")]
use gpt;

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
    
    let diskpath = std::path::Path::new("/dev/").join(device.name.to_string());
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