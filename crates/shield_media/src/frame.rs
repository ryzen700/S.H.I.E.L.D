use std::sync::Arc;

/// Raw uncompressed 32-bit RGBA video frame suitable for direct GPU texture uploading.
#[derive(Clone, Debug)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub timestamp_ms: u64,
    pub data: Arc<Vec<u8>>,
}

impl VideoFrame {
    pub fn new(width: u32, height: u32, timestamp_ms: u64, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            timestamp_ms,
            data: Arc::new(data),
        }
    }

    /// Size of the pixel buffer in bytes (width * height * 4 for RGBA).
    pub fn byte_len(&self) -> usize {
        (self.width as usize) * (self.height as usize) * 4
    }

    /// Validates whether the pixel buffer matches the specified dimensions.
    pub fn is_valid(&self) -> bool {
        self.data.len() == self.byte_len() && self.width > 0 && self.height > 0
    }
}
