use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaySession {
    pub id: String,
    pub game_id: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: u64,
}

impl PlaySession {
    pub fn new(id: impl Into<String>, game_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            game_id: game_id.into(),
            started_at: Utc::now(),
            ended_at: None,
            duration_seconds: 0,
        }
    }

    pub fn complete(&mut self, ended_at: DateTime<Utc>) {
        let duration = ended_at
            .signed_duration_since(self.started_at)
            .num_seconds();
        self.ended_at = Some(ended_at);
        self.duration_seconds = if duration > 0 { duration as u64 } else { 0 };
    }
}
