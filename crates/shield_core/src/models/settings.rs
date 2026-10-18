use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettings {
    pub download_directory: PathBuf,
    pub auto_scan_libraries: bool,
    pub custom_scan_directories: Vec<PathBuf>,
    pub steamgriddb_api_key: Option<String>,
    pub default_runner: String,
    pub max_download_speed_bps: Option<u64>,
    pub max_upload_speed_bps: Option<u64>,
    pub close_to_tray: bool,
    pub start_minimized: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        let download_dir = dirs::download_dir()
            .map(|p| p.join("SHIELD_Downloads"))
            .unwrap_or_else(|| PathBuf::from("/tmp/SHIELD_Downloads"));

        Self {
            download_directory: download_dir,
            auto_scan_libraries: true,
            custom_scan_directories: Vec::new(),
            steamgriddb_api_key: None,
            default_runner: "native".to_string(),
            max_download_speed_bps: None,
            max_upload_speed_bps: None,
            close_to_tray: false,
            start_minimized: false,
        }
    }
}
