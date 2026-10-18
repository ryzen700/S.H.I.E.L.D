use crate::models::{Platform, TorrentStatus};
use std::path::PathBuf;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum ShieldEvent {
    GameLaunched {
        game_id: String,
        title: String,
    },
    GameExited {
        game_id: String,
        session_duration_seconds: u64,
    },
    TorrentAdded {
        info_hash: String,
        name: String,
    },
    TorrentUpdated {
        info_hash: String,
        progress: f32,
        download_speed_bps: u64,
        status: TorrentStatus,
    },
    TorrentFinished {
        info_hash: String,
        name: String,
        save_path: PathBuf,
    },
    LibraryScanStarted {
        platform: Platform,
    },
    LibraryScanCompleted {
        platform: Platform,
        games_found: usize,
    },
    SettingsUpdated,
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<ShieldEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ShieldEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: ShieldEvent) {
        // Discard errors if there are no active listeners
        let _ = self.sender.send(event);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}
