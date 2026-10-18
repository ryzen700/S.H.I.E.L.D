use crate::error::MediaError;
use crate::ffmpeg::FfmpegResolver;
use crate::frame::VideoFrame;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Ended,
    Error,
}

pub struct VideoPlayer {
    source: String,
    width: u32,
    height: u32,
    state: Arc<Mutex<PlaybackState>>,
    current_frame: Arc<Mutex<Option<VideoFrame>>>,
    stop_signal: Arc<AtomicBool>,
}

impl VideoPlayer {
    pub fn new(source: impl Into<String>, width: u32, height: u32) -> Result<Self, MediaError> {
        if width == 0 || height == 0 {
            return Err(MediaError::InvalidDimensions(width, height));
        }

        Ok(Self {
            source: source.into(),
            width,
            height,
            state: Arc::new(Mutex::new(PlaybackState::Stopped)),
            current_frame: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn current_frame(&self) -> Option<VideoFrame> {
        self.current_frame.try_lock().ok().and_then(|guard| guard.clone())
    }

    pub async fn state(&self) -> PlaybackState {
        *self.state.lock().await
    }

    pub fn start(&self) {
        let source = self.source.clone();
        let width = self.width;
        let height = self.height;
        let state = self.state.clone();
        let current_frame = self.current_frame.clone();
        let stop_signal = self.stop_signal.clone();

        stop_signal.store(false, Ordering::SeqCst);

        tokio::spawn(async move {
            *state.lock().await = PlaybackState::Playing;

            let ffmpeg_binary = match FfmpegResolver::resolve_binary() {
                Ok(path) => path,
                Err(e) => {
                    error!("Cannot start video player: {}", e);
                    *state.lock().await = PlaybackState::Error;
                    return;
                }
            };

            info!("Starting FFmpeg video decode for '{}' at {}x{}", source, width, height);

            let mut child = match Command::new(ffmpeg_binary)
                .args([
                    "-re",
                    "-loglevel", "error",
                    "-i", &source,
                    "-vf", &format!("scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2:black", width, height, width, height),
                    "-f", "rawvideo",
                    "-pix_fmt", "rgba",
                    "-r", "30",
                    "pipe:1"
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => child,
                Err(err) => {
                    error!("Failed to spawn FFmpeg process: {}", err);
                    *state.lock().await = PlaybackState::Error;
                    return;
                }
            };

            let mut stdout = match child.stdout.take() {
                Some(out) => out,
                None => {
                    error!("FFmpeg stdout pipe unavailable");
                    *state.lock().await = PlaybackState::Error;
                    return;
                }
            };

            let frame_bytes = (width as usize) * (height as usize) * 4;
            let mut buffer = vec![0u8; frame_bytes];
            let mut timestamp_ms = 0u64;

            while !stop_signal.load(Ordering::SeqCst) {
                match stdout.read_exact(&mut buffer).await {
                    Ok(_) => {
                        let frame = VideoFrame::new(width, height, timestamp_ms, buffer.clone());
                        *current_frame.lock().await = Some(frame);
                        timestamp_ms += 33; // ~30 fps
                    }
                    Err(e) => {
                        debug!("Video stream ended or interrupted: {}", e);
                        break;
                    }
                }
            }

            let _ = child.kill().await;
            let mut current_state = state.lock().await;
            if *current_state != PlaybackState::Stopped {
                *current_state = PlaybackState::Ended;
            }
        });
    }

    pub async fn stop(&self) {
        self.stop_signal.store(true, Ordering::SeqCst);
        *self.state.lock().await = PlaybackState::Stopped;
        *self.current_frame.lock().await = None;
    }
}
