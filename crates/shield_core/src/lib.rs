pub mod db;
pub mod events;
pub mod models;

pub use db::{DatabaseError, ShieldDatabase};
pub use events::{EventBus, ShieldEvent};
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_game_crud_and_search() {
        let db = ShieldDatabase::open_in_memory().expect("failed to open in-memory db");

        let mut game1 = Game::new("game-1", "Half-Life 2", Platform::Steam);
        game1.install_path = Some(PathBuf::from("/games/hl2"));
        game1.executable_path = Some(PathBuf::from("/games/hl2/hl2_linux"));
        game1.genres = vec!["FPS".to_string(), "Sci-Fi".to_string()];
        game1.tags = vec!["Favorite".to_string()];

        let mut game2 = Game::new("game-2", "Cyberpunk 2077", Platform::Gog);
        game2.genres = vec!["RPG".to_string(), "Sci-Fi".to_string()];

        db.upsert_game(&game1).expect("failed to insert game1");
        db.upsert_game(&game2).expect("failed to insert game2");

        // Verify retrieval
        let fetched = db.get_game("game-1").expect("db error").expect("game not found");
        assert_eq!(fetched.title, "Half-Life 2");
        assert_eq!(fetched.platform, Platform::Steam);
        assert_eq!(fetched.genres, vec!["FPS".to_string(), "Sci-Fi".to_string()]);

        // Search
        let search_hl = db.search_games("Half").expect("search error");
        assert_eq!(search_hl.len(), 1);
        assert_eq!(search_hl[0].id, "game-1");

        let search_scifi = db.search_games("Sci-Fi").expect("search error");
        assert_eq!(search_scifi.len(), 2);

        // Update playtime
        db.update_playtime("game-1", 3600).expect("failed to update playtime");
        let updated = db.get_game("game-1").expect("db error").unwrap();
        assert_eq!(updated.playtime_seconds, 3600);
        assert!(updated.last_played.is_some());

        // Favorite
        db.set_favorite("game-1", true).expect("failed to set favorite");
        let fav = db.get_game("game-1").expect("db error").unwrap();
        assert!(fav.favorite);

        // Delete
        db.delete_game("game-2").expect("delete error");
        assert!(db.get_game("game-2").expect("db error").is_none());
    }

    #[test]
    fn test_torrent_crud() {
        let db = ShieldDatabase::open_in_memory().expect("failed to open in-memory db");

        let mut task = TorrentTask::new("hash123", "Test Game Repack", PathBuf::from("/downloads"));
        task.total_bytes = 10_000_000;
        task.downloaded_bytes = 5_000_000;
        task.progress = 0.5;
        task.status = TorrentStatus::Downloading;

        db.upsert_torrent(&task).expect("failed to insert torrent");

        let fetched = db.get_torrent("hash123").expect("db error").expect("torrent not found");
        assert_eq!(fetched.name, "Test Game Repack");
        assert_eq!(fetched.progress, 0.5);
        assert_eq!(fetched.status, TorrentStatus::Downloading);

        let all = db.get_all_torrents().expect("db error");
        assert_eq!(all.len(), 1);

        db.delete_torrent("hash123").expect("delete error");
        assert!(db.get_torrent("hash123").expect("db error").is_none());
    }

    #[test]
    fn test_play_sessions() {
        let db = ShieldDatabase::open_in_memory().expect("failed to open in-memory db");
        let game = Game::new("game-test", "Test Game", Platform::Custom);
        db.upsert_game(&game).expect("failed to insert game");

        let mut session = PlaySession::new("sess-1", "game-test");
        session.duration_seconds = 1800;
        db.record_session(&session).expect("record session error");

        let sessions = db.get_sessions_for_game("game-test").expect("get sessions error");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].duration_seconds, 1800);
    }

    #[test]
    fn test_event_bus() {
        let bus = EventBus::default();
        let mut rx = bus.subscribe();

        bus.publish(ShieldEvent::GameLaunched {
            game_id: "g1".to_string(),
            title: "Game 1".to_string(),
        });

        let event = rx.try_recv().expect("event receive error");
        match event {
            ShieldEvent::GameLaunched { game_id, title } => {
                assert_eq!(game_id, "g1");
                assert_eq!(title, "Game 1");
            }
            _ => panic!("unexpected event received"),
        }
    }
}
