use crate::error::MediaError;
use std::env;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Locates the bundled FFmpeg codec binary or falls back to system installations.
pub struct FfmpegResolver;

impl FfmpegResolver {
    /// Discovers the FFmpeg executable prioritizing bundled application release paths.
    pub fn resolve_binary() -> Result<PathBuf, MediaError> {
        // 1. Check override environment variable
        if let Ok(env_path) = env::var("SHIELD_FFMPEG_PATH") {
            let p = PathBuf::from(env_path);
            if p.exists() {
                debug!("Using FFmpeg from SHIELD_FFMPEG_PATH: {:?}", p);
                return Ok(p);
            }
        }

        // 2. Check bundled release path relative to current executable
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let candidates = [
                    exe_dir.join("ffmpeg"),
                    exe_dir.join("ffmpeg.exe"),
                    exe_dir.join("bin").join("ffmpeg"),
                    exe_dir.join("bin").join("ffmpeg.exe"),
                    exe_dir.join("lib").join("ffmpeg"),
                    exe_dir.join("../lib/shield/ffmpeg"),
                    exe_dir.join("../share/shield/ffmpeg"),
                ];

                for candidate in &candidates {
                    if candidate.exists() {
                        info!("Found bundled release FFmpeg codec: {:?}", candidate);
                        return Ok(candidate.clone());
                    }
                }
            }
        }

        // 3. Check well-known system PATH locations
        let system_paths = [
            "/usr/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/opt/homebrew/bin/ffmpeg",
        ];

        for path in system_paths {
            let p = Path::new(path);
            if p.exists() {
                debug!("Found system FFmpeg at {:?}", p);
                return Ok(p.to_path_buf());
            }
        }

        // 4. Check general PATH via standard command search
        if let Ok(path) = which_in_path("ffmpeg") {
            return Ok(path);
        }

        Err(MediaError::CodecNotFound(PathBuf::from("ffmpeg")))
    }
}

fn which_in_path(binary_name: &str) -> Result<PathBuf, ()> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            let full_path = dir.join(binary_name);
            if full_path.is_file() {
                return Ok(full_path);
            }
        }
    }
    Err(())
}
