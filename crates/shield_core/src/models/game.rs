use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Steam,
    Epic,
    Gog,
    Torrent,
    Custom,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Steam => "steam",
            Self::Epic => "epic",
            Self::Gog => "gog",
            Self::Torrent => "torrent",
            Self::Custom => "custom",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "steam" => Self::Steam,
            "epic" => Self::Epic,
            "gog" => Self::Gog,
            "torrent" => Self::Torrent,
            _ => Self::Custom,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunnerType {
    Native,
    SteamProtocol,
    Proton,
    Wine,
}

impl RunnerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::SteamProtocol => "steam_protocol",
            Self::Proton => "proton",
            Self::Wine => "wine",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "native" => Self::Native,
            "steam_protocol" => Self::SteamProtocol,
            "proton" => Self::Proton,
            "wine" => Self::Wine,
            _ => Self::Native,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallStatus {
    Installed,
    NotInstalled,
    Installing,
    Downloading,
    UpdateAvailable,
}

impl InstallStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::NotInstalled => "not_installed",
            Self::Installing => "installing",
            Self::Downloading => "downloading",
            Self::UpdateAvailable => "update_available",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "installed" => Self::Installed,
            "installing" => Self::Installing,
            "downloading" => Self::Downloading,
            "update_available" => Self::UpdateAvailable,
            _ => Self::NotInstalled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunnerConfig {
    pub prefix_path: Option<PathBuf>,
    pub proton_path: Option<PathBuf>,
    pub enable_gamemode: bool,
    pub enable_mangohud: bool,
    pub enable_gamescope: bool,
    pub custom_args: Vec<String>,
    pub environment_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub platform: Platform,
    pub external_id: Option<String>,
    pub install_path: Option<PathBuf>,
    pub executable_path: Option<PathBuf>,
    pub runner_type: RunnerType,
    pub runner_config: RunnerConfig,
    pub cover_path: Option<PathBuf>,
    pub banner_path: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
    pub description: Option<String>,
    pub genres: Vec<String>,
    pub release_date: Option<String>,
    pub playtime_seconds: u64,
    pub last_played: Option<DateTime<Utc>>,
    pub install_status: InstallStatus,
    pub favorite: bool,
    pub tags: Vec<String>,
}

impl Game {
    pub fn new(id: impl Into<String>, title: impl Into<String>, platform: Platform) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            platform,
            external_id: None,
            install_path: None,
            executable_path: None,
            runner_type: RunnerType::Native,
            runner_config: RunnerConfig::default(),
            cover_path: None,
            banner_path: None,
            icon_path: None,
            description: None,
            genres: Vec::new(),
            release_date: None,
            playtime_seconds: 0,
            last_played: None,
            install_status: InstallStatus::Installed,
            favorite: false,
            tags: Vec::new(),
        }
    }

    pub fn format_playtime(&self) -> String {
        let hours = self.playtime_seconds / 3600;
        let minutes = (self.playtime_seconds % 3600) / 60;
        if hours > 0 {
            format!("{} ч {} мин", hours, minutes)
        } else if minutes > 0 {
            format!("{} мин", minutes)
        } else {
            "Не играли".to_string()
        }
    }
}
