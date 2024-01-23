use thiserror::Error;

#[derive(Error, Debug)]
pub enum DrivesError {
    #[error("failed to access/open file {filename:?}")]
    FileAccessError { filename: String },
    #[error("failed to read file {filename:?}")]
    FileReadError { filename: String },
    #[error("Couldn't get file type for {filename:?}")]
    FileTypeError { filename: String },
    #[error("failed to get name for DirEntry")]
    NameFromDirEntryFailed,
    #[error("failed to append path")]
    PathAppendFailed,
    #[error("failed to convert file content to u64")]
    ConversionToU64Failed,
    #[error("failed to convert file content to u32")]
    ConversionToU32Failed,
    #[error("failed to access directory {directory:?}")]
    DiraccessError { directory: String },
    #[error("reading mounts from /proc/mounts failed")]
    ReadingMountsFailed,
}
