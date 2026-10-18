use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("FFmpeg binary/codec not found at {0:?} or in system PATH")]
    CodecNotFound(PathBuf),

    #[error("Failed to execute FFmpeg process: {0}")]
    ProcessSpawnFailed(String),

    #[error("IO error during media stream processing: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid media stream parameters: width={0}, height={1}")]
    InvalidDimensions(u32, u32),

    #[error("Media stream terminated unexpectedly: {0}")]
    StreamTerminated(String),

    #[error("Failed to decode video frame: {0}")]
    DecodeError(String),
}
