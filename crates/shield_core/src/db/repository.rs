use crate::db::schema::{CREATE_MIGRATIONS_TABLE, CURRENT_SCHEMA_VERSION, MIGRATION_V1};
use crate::models::{
    AppSettings, Game, InstallStatus, Platform, PlaySession, RunnerConfig, RunnerType,
    TorrentStatus, TorrentTask,
};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Database lock poisoned")]
    LockPoisoned,
    #[error("Migration error: {0}")]
    Migration(String),
}

#[derive(Clone)]
pub struct ShieldDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl ShieldDatabase {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        if let Some(parent) = db_path.as_ref().parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.run_migrations()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self, DatabaseError> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        conn.execute_batch(CREATE_MIGRATIONS_TABLE)?;

        let mut stmt = conn.prepare("SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1")?;
        let current_version: Option<i32> = stmt.query_row([], |row| row.get(0)).optional()?;

        let version = current_version.unwrap_or(0);
        if version < CURRENT_SCHEMA_VERSION {
            conn.execute_batch(MIGRATION_V1)?;
            conn.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                params![CURRENT_SCHEMA_VERSION, Utc::now().to_rfc3339()],
            )?;
        }

        Ok(())
    }

    // --- GAME OPERATIONS ---

    pub fn upsert_game(&self, game: &Game) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let runner_config_json = serde_json::to_string(&game.runner_config)?;
        let genres_json = serde_json::to_string(&game.genres)?;
        let tags_json = serde_json::to_string(&game.tags)?;
        let last_played_str = game.last_played.map(|dt| dt.to_rfc3339());
        let install_path_str = game.install_path.as_ref().map(|p| p.to_string_lossy().to_string());
        let exec_path_str = game.executable_path.as_ref().map(|p| p.to_string_lossy().to_string());
        let cover_str = game.cover_path.as_ref().map(|p| p.to_string_lossy().to_string());
        let banner_str = game.banner_path.as_ref().map(|p| p.to_string_lossy().to_string());
        let icon_str = game.icon_path.as_ref().map(|p| p.to_string_lossy().to_string());

        conn.execute(
            r#"
            INSERT INTO games (
                id, title, platform, external_id, install_path, executable_path,
                runner_type, runner_config_json, cover_path, banner_path, icon_path,
                description, genres_json, release_date, playtime_seconds, last_played,
                install_status, favorite, tags_json
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19
            )
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                platform = excluded.platform,
                external_id = excluded.external_id,
                install_path = COALESCE(excluded.install_path, games.install_path),
                executable_path = COALESCE(excluded.executable_path, games.executable_path),
                runner_type = excluded.runner_type,
                runner_config_json = excluded.runner_config_json,
                cover_path = COALESCE(excluded.cover_path, games.cover_path),
                banner_path = COALESCE(excluded.banner_path, games.banner_path),
                icon_path = COALESCE(excluded.icon_path, games.icon_path),
                description = COALESCE(excluded.description, games.description),
                genres_json = excluded.genres_json,
                release_date = COALESCE(excluded.release_date, games.release_date),
                install_status = excluded.install_status,
                tags_json = excluded.tags_json
            "#,
            params![
                game.id,
                game.title,
                game.platform.as_str(),
                game.external_id,
                install_path_str,
                exec_path_str,
                game.runner_type.as_str(),
                runner_config_json,
                cover_str,
                banner_str,
                icon_str,
                game.description,
                genres_json,
                game.release_date,
                game.playtime_seconds,
                last_played_str,
                game.install_status.as_str(),
                if game.favorite { 1 } else { 0 },
                tags_json,
            ],
        )?;

        Ok(())
    }

    pub fn get_game(&self, id: &str) -> Result<Option<Game>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT id, title, platform, external_id, install_path, executable_path,
                    runner_type, runner_config_json, cover_path, banner_path, icon_path,
                    description, genres_json, release_date, playtime_seconds, last_played,
                    install_status, favorite, tags_json
             FROM games WHERE id = ?1",
        )?;

        let game = stmt
            .query_row(params![id], Self::map_game_row)
            .optional()?;

        Ok(game)
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT id, title, platform, external_id, install_path, executable_path,
                    runner_type, runner_config_json, cover_path, banner_path, icon_path,
                    description, genres_json, release_date, playtime_seconds, last_played,
                    install_status, favorite, tags_json
             FROM games ORDER BY title ASC",
        )?;

        let rows = stmt.query_map([], Self::map_game_row)?;
        let mut games = Vec::new();
        for row in rows {
            games.push(row?);
        }
        Ok(games)
    }

    pub fn get_games_by_platform(&self, platform: Platform) -> Result<Vec<Game>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT id, title, platform, external_id, install_path, executable_path,
                    runner_type, runner_config_json, cover_path, banner_path, icon_path,
                    description, genres_json, release_date, playtime_seconds, last_played,
                    install_status, favorite, tags_json
             FROM games WHERE platform = ?1 ORDER BY title ASC",
        )?;

        let rows = stmt.query_map(params![platform.as_str()], Self::map_game_row)?;
        let mut games = Vec::new();
        for row in rows {
            games.push(row?);
        }
        Ok(games)
    }

    pub fn search_games(&self, query: &str) -> Result<Vec<Game>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, title, platform, external_id, install_path, executable_path,
                    runner_type, runner_config_json, cover_path, banner_path, icon_path,
                    description, genres_json, release_date, playtime_seconds, last_played,
                    install_status, favorite, tags_json
             FROM games WHERE title LIKE ?1 OR genres_json LIKE ?1 ORDER BY title ASC",
        )?;

        let rows = stmt.query_map(params![pattern], Self::map_game_row)?;
        let mut games = Vec::new();
        for row in rows {
            games.push(row?);
        }
        Ok(games)
    }

    pub fn update_playtime(&self, game_id: &str, add_seconds: u64) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let now_str = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE games SET playtime_seconds = playtime_seconds + ?1, last_played = ?2 WHERE id = ?3",
            params![add_seconds, now_str, game_id],
        )?;
        Ok(())
    }

    pub fn set_favorite(&self, game_id: &str, favorite: bool) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        conn.execute(
            "UPDATE games SET favorite = ?1 WHERE id = ?2",
            params![if favorite { 1 } else { 0 }, game_id],
        )?;
        Ok(())
    }

    pub fn delete_game(&self, game_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        conn.execute("DELETE FROM games WHERE id = ?1", params![game_id])?;
        Ok(())
    }

    fn map_game_row(row: &rusqlite::Row) -> Result<Game, rusqlite::Error> {
        let platform_str: String = row.get(2)?;
        let runner_type_str: String = row.get(6)?;
        let runner_config_json: String = row.get(7)?;
        let genres_json: String = row.get(12)?;
        let last_played_str: Option<String> = row.get(15)?;
        let status_str: String = row.get(16)?;
        let favorite_int: i32 = row.get(17)?;
        let tags_json: String = row.get(18)?;

        let runner_config: RunnerConfig = serde_json::from_str(&runner_config_json).unwrap_or_default();
        let genres: Vec<String> = serde_json::from_str(&genres_json).unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        let last_played = last_played_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc)));

        let install_path: Option<String> = row.get(4)?;
        let executable_path: Option<String> = row.get(5)?;
        let cover_path: Option<String> = row.get(8)?;
        let banner_path: Option<String> = row.get(9)?;
        let icon_path: Option<String> = row.get(10)?;

        Ok(Game {
            id: row.get(0)?,
            title: row.get(1)?,
            platform: Platform::from_str_lossy(&platform_str),
            external_id: row.get(3)?,
            install_path: install_path.map(PathBuf::from),
            executable_path: executable_path.map(PathBuf::from),
            runner_type: RunnerType::from_str_lossy(&runner_type_str),
            runner_config,
            cover_path: cover_path.map(PathBuf::from),
            banner_path: banner_path.map(PathBuf::from),
            icon_path: icon_path.map(PathBuf::from),
            description: row.get(11)?,
            genres,
            release_date: row.get(13)?,
            playtime_seconds: row.get(14)?,
            last_played,
            install_status: InstallStatus::from_str_lossy(&status_str),
            favorite: favorite_int != 0,
            tags,
        })
    }

    // --- TORRENT OPERATIONS ---

    pub fn upsert_torrent(&self, task: &TorrentTask) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let added_at_str = task.added_at.to_rfc3339();
        let completed_at_str = task.completed_at.map(|dt| dt.to_rfc3339());
        let save_path_str = task.save_path.to_string_lossy().to_string();

        conn.execute(
            r#"
            INSERT INTO torrents (
                info_hash, name, magnet_uri, save_path, total_bytes, downloaded_bytes,
                uploaded_bytes, download_speed_bps, upload_speed_bps, progress,
                status, added_at, completed_at, error_message
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14
            )
            ON CONFLICT(info_hash) DO UPDATE SET
                total_bytes = excluded.total_bytes,
                downloaded_bytes = excluded.downloaded_bytes,
                uploaded_bytes = excluded.uploaded_bytes,
                download_speed_bps = excluded.download_speed_bps,
                upload_speed_bps = excluded.upload_speed_bps,
                progress = excluded.progress,
                status = excluded.status,
                completed_at = excluded.completed_at,
                error_message = excluded.error_message
            "#,
            params![
                task.info_hash,
                task.name,
                task.magnet_uri,
                save_path_str,
                task.total_bytes,
                task.downloaded_bytes,
                task.uploaded_bytes,
                task.download_speed_bps,
                task.upload_speed_bps,
                task.progress,
                task.status.as_str(),
                added_at_str,
                completed_at_str,
                task.error_message,
            ],
        )?;

        Ok(())
    }

    pub fn get_torrent(&self, info_hash: &str) -> Result<Option<TorrentTask>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT info_hash, name, magnet_uri, save_path, total_bytes, downloaded_bytes,
                    uploaded_bytes, download_speed_bps, upload_speed_bps, progress,
                    status, added_at, completed_at, error_message
             FROM torrents WHERE info_hash = ?1",
        )?;

        let task = stmt
            .query_row(params![info_hash], Self::map_torrent_row)
            .optional()?;

        Ok(task)
    }

    pub fn get_all_torrents(&self) -> Result<Vec<TorrentTask>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT info_hash, name, magnet_uri, save_path, total_bytes, downloaded_bytes,
                    uploaded_bytes, download_speed_bps, upload_speed_bps, progress,
                    status, added_at, completed_at, error_message
             FROM torrents ORDER BY added_at DESC",
        )?;

        let rows = stmt.query_map([], Self::map_torrent_row)?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    pub fn delete_torrent(&self, info_hash: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        conn.execute("DELETE FROM torrents WHERE info_hash = ?1", params![info_hash])?;
        Ok(())
    }

    fn map_torrent_row(row: &rusqlite::Row) -> Result<TorrentTask, rusqlite::Error> {
        let save_path_str: String = row.get(3)?;
        let status_str: String = row.get(10)?;
        let added_at_str: String = row.get(11)?;
        let completed_at_str: Option<String> = row.get(12)?;

        let added_at = DateTime::parse_from_rfc3339(&added_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let completed_at = completed_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc)));

        Ok(TorrentTask {
            info_hash: row.get(0)?,
            name: row.get(1)?,
            magnet_uri: row.get(2)?,
            save_path: PathBuf::from(save_path_str),
            total_bytes: row.get(4)?,
            downloaded_bytes: row.get(5)?,
            uploaded_bytes: row.get(6)?,
            download_speed_bps: row.get(7)?,
            upload_speed_bps: row.get(8)?,
            progress: row.get(9)?,
            status: TorrentStatus::from_str_lossy(&status_str),
            added_at,
            completed_at,
            error_message: row.get(13)?,
        })
    }

    // --- PLAY SESSION OPERATIONS ---

    pub fn record_session(&self, session: &PlaySession) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let started_at_str = session.started_at.to_rfc3339();
        let ended_at_str = session.ended_at.map(|dt| dt.to_rfc3339());

        conn.execute(
            r#"
            INSERT INTO play_sessions (id, game_id, started_at, ended_at, duration_seconds)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                ended_at = excluded.ended_at,
                duration_seconds = excluded.duration_seconds
            "#,
            params![
                session.id,
                session.game_id,
                started_at_str,
                ended_at_str,
                session.duration_seconds,
            ],
        )?;

        Ok(())
    }

    pub fn get_sessions_for_game(&self, game_id: &str) -> Result<Vec<PlaySession>, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare(
            "SELECT id, game_id, started_at, ended_at, duration_seconds
             FROM play_sessions WHERE game_id = ?1 ORDER BY started_at DESC",
        )?;

        let rows = stmt.query_map(params![game_id], |row| {
            let started_at_str: String = row.get(2)?;
            let ended_at_str: Option<String> = row.get(3)?;
            let started_at = DateTime::parse_from_rfc3339(&started_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let ended_at = ended_at_str
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc)));

            Ok(PlaySession {
                id: row.get(0)?,
                game_id: row.get(1)?,
                started_at,
                ended_at,
                duration_seconds: row.get(4)?,
            })
        })?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row?);
        }
        Ok(sessions)
    }

    // --- APP SETTINGS OPERATIONS ---

    pub fn get_settings(&self) -> Result<AppSettings, DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let mut stmt = conn.prepare("SELECT value_json FROM app_settings WHERE key = 'global'")?;
        let json_opt: Option<String> = stmt.query_row([], |row| row.get(0)).optional()?;

        match json_opt {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(AppSettings::default()),
        }
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().map_err(|_| DatabaseError::LockPoisoned)?;
        let json = serde_json::to_string(settings)?;
        conn.execute(
            "INSERT INTO app_settings (key, value_json) VALUES ('global', ?1)
             ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json",
            params![json],
        )?;
        Ok(())
    }
}
