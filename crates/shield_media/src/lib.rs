pub mod error;
pub mod ffmpeg;
pub mod frame;
pub mod player;

pub use error::MediaError;
pub use ffmpeg::FfmpegResolver;
pub use frame::VideoFrame;
pub use player::{PlaybackState, VideoPlayer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_frame_validation() {
        let frame = VideoFrame::new(640, 360, 0, vec![255; 640 * 360 * 4]);
        assert!(frame.is_valid());
        assert_eq!(frame.byte_len(), 640 * 360 * 4);

        let invalid_frame = VideoFrame::new(640, 360, 0, vec![0; 100]);
        assert!(!invalid_frame.is_valid());
    }

    #[test]
    fn test_ffmpeg_resolver() {
        let result = FfmpegResolver::resolve_binary();
        assert!(result.is_ok(), "Expected FFmpeg binary to be found on system");
    }

    #[tokio::test]
    async fn test_player_initialization() {
        let player = VideoPlayer::new("sample.mp4", 1280, 720).expect("failed to init player");
        assert_eq!(player.state().await, PlaybackState::Stopped);
        assert!(player.current_frame().is_none());
    }
}
