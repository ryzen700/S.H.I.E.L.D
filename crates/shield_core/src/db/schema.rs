pub const CURRENT_SCHEMA_VERSION: i32 = 1;

pub const CREATE_MIGRATIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
"#;

pub const MIGRATION_V1: &str = r#"
CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    platform TEXT NOT NULL,
    external_id TEXT,
    install_path TEXT,
    executable_path TEXT,
    runner_type TEXT NOT NULL,
    runner_config_json TEXT NOT NULL,
    cover_path TEXT,
    banner_path TEXT,
    icon_path TEXT,
    description TEXT,
    genres_json TEXT NOT NULL,
    release_date TEXT,
    playtime_seconds INTEGER NOT NULL DEFAULT 0,
    last_played TEXT,
    install_status TEXT NOT NULL DEFAULT 'installed',
    favorite INTEGER NOT NULL DEFAULT 0,
    tags_json TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_games_platform ON games(platform);
CREATE INDEX IF NOT EXISTS idx_games_title ON games(title);
CREATE INDEX IF NOT EXISTS idx_games_favorite ON games(favorite);

CREATE TABLE IF NOT EXISTS play_sessions (
    id TEXT PRIMARY KEY,
    game_id TEXT NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sessions_game ON play_sessions(game_id);

CREATE TABLE IF NOT EXISTS torrents (
    info_hash TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    magnet_uri TEXT,
    save_path TEXT NOT NULL,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    uploaded_bytes INTEGER NOT NULL DEFAULT 0,
    download_speed_bps INTEGER NOT NULL DEFAULT 0,
    upload_speed_bps INTEGER NOT NULL DEFAULT 0,
    progress REAL NOT NULL DEFAULT 0.0,
    status TEXT NOT NULL DEFAULT 'queued',
    added_at TEXT NOT NULL,
    completed_at TEXT,
    error_message TEXT
);

CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL
);
"#;
