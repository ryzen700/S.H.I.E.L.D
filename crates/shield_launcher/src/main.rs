use shield_core::ShieldDatabase;
use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting S.H.I.E.L.D Game Launcher (Pure Rust, Native GPU Renderer)...");

    let data_dir = dirs::data_dir()
        .map(|d| d.join("shield"))
        .unwrap_or_else(|| PathBuf::from(".shield"));
    std::fs::create_dir_all(&data_dir).ok();

    let db_path = data_dir.join("shield.db");
    let db = match ShieldDatabase::new(&db_path) {
        Ok(db) => {
            tracing::info!("Database initialized at {:?}", db_path);
            db
        }
        Err(e) => {
            tracing::error!("Failed to open database at {:?}: {}, falling back to in-memory", db_path, e);
            ShieldDatabase::open_in_memory().expect("in-memory db fallback failed")
        }
    };

    let settings = db.get_settings().unwrap_or_default();
    tracing::info!(
        "Loaded settings: download_dir = {:?}",
        settings.download_directory
    );

    shield_ui::run_shield_app(db, settings);
}
