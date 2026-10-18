use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TorrentStatus {
    Queued,
    Downloading,
    Seeding,
    Paused,
    Completed,
    Error,
}

impl TorrentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Downloading => "downloading",
            Self::Seeding => "seeding",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Error => "error",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "queued" => Self::Queued,
            "downloading" => Self::Downloading,
            "seeding" => Self::Seeding,
            "paused" => Self::Paused,
            "completed" => Self::Completed,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorrentTask {
    pub info_hash: String,
    pub name: String,
    pub magnet_uri: Option<String>,
    pub save_path: PathBuf,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub uploaded_bytes: u64,
    pub download_speed_bps: u64,
    pub upload_speed_bps: u64,
    pub progress: f32,
    pub status: TorrentStatus,
    pub added_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

impl TorrentTask {
    pub fn new(info_hash: impl Into<String>, name: impl Into<String>, save_path: PathBuf) -> Self {
        Self {
            info_hash: info_hash.into(),
            name: name.into(),
            magnet_uri: None,
            save_path,
            total_bytes: 0,
            downloaded_bytes: 0,
            uploaded_bytes: 0,
            download_speed_bps: 0,
            upload_speed_bps: 0,
            progress: 0.0,
            status: TorrentStatus::Queued,
            added_at: Utc::now(),
            completed_at: None,
            error_message: None,
        }
    }

    pub fn format_speed(bps: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if bps >= GB {
            format!("{:.2} GB/s", bps as f64 / GB as f64)
        } else if bps >= MB {
            format!("{:.2} MB/s", bps as f64 / MB as f64)
        } else if bps >= KB {
            format!("{:.1} KB/s", bps as f64 / KB as f64)
        } else {
            format!("{} B/s", bps)
        }
    }

    pub fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.0} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}
