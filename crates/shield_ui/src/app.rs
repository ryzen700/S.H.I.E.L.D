use crate::components::*;
use crate::theme::HydraTheme;
use crate::views::*;
use gpui::*;
use shield_core::models::{AppSettings, Game, Platform, TorrentStatus, TorrentTask};
use shield_core::ShieldDatabase;
use std::path::PathBuf;

pub struct ShieldApp {
    pub db: ShieldDatabase,
    pub settings: AppSettings,
    pub active_tab: NavigationTab,
    pub games: Vec<Game>,
    pub torrents: Vec<TorrentTask>,
    pub selected_game: Option<Game>,
}

impl ShieldApp {
    pub fn new(db: ShieldDatabase, settings: AppSettings) -> Self {
        let mut games = db.get_all_games().unwrap_or_default();
        if games.is_empty() {
            let mut game1 = Game::new("hl2", "Half-Life 2", Platform::Steam);
            game1.executable_path = Some(PathBuf::from("/games/hl2/hl2_linux"));
            game1.playtime_seconds = 7200;
            game1.favorite = true;

            let mut game2 = Game::new("cp2077", "Cyberpunk 2077", Platform::Gog);
            game2.executable_path = Some(PathBuf::from("/games/cp2077/bin/x64/Cyberpunk2077.exe"));
            game2.playtime_seconds = 45000;
            game2.favorite = true;

            let game3 = Game::new("witcher3", "The Witcher 3: Wild Hunt", Platform::Steam);
            let game4 = Game::new("eldenring", "ELDEN RING", Platform::Steam);

            games = vec![game1, game2, game3, game4];
        }

        let mut torrents = db.get_all_torrents().unwrap_or_default();
        if torrents.is_empty() {
            let mut torrent1 = TorrentTask::new("hash-hl2", "Half-Life 2 Update Package", PathBuf::from("/downloads"));
            torrent1.progress = 0.68;
            torrent1.downloaded_bytes = 4_800_000_000;
            torrent1.total_bytes = 7_000_000_000;
            torrent1.download_speed_bps = 18_500_000;
            torrent1.status = TorrentStatus::Downloading;
            torrents = vec![torrent1];
        }

        Self {
            db,
            settings,
            active_tab: NavigationTab::Catalogue,
            games,
            torrents,
            selected_game: None,
        }
    }
}

impl Render for ShieldApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = HydraTheme::default();
        let installed_count = self.games.iter().filter(|g| g.executable_path.is_some()).count();
        let downloads_count = self.torrents.len();

        let content_element: AnyElement = match self.active_tab {
            NavigationTab::Catalogue => render_catalogue(&self.games).into_any_element(),
            NavigationTab::Library => render_library(&self.games, LibraryFilter::All).into_any_element(),
            NavigationTab::Downloads => render_downloads(&self.torrents).into_any_element(),
            NavigationTab::Settings => render_settings().into_any_element(),
        };

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(theme.bg_main)
            // Left Sidebar matching Hydra
            .child(render_sidebar(self.active_tab, installed_count, downloads_count))
            // Right Main Content Area
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(render_header(""))
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            .bg(theme.bg_main)
                            .child(content_element)
                    )
            )
    }
}

pub fn run_shield_app(db: ShieldDatabase, settings: AppSettings) {
    Application::new().run(move |cx: &mut App| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::default(),
                size: size(px(1280.0), px(800.0)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("S.H.I.E.L.D. Game Launcher".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        cx.open_window(window_options, move |_window, cx| {
            let app_instance = ShieldApp::new(db, settings);
            cx.new(|_| app_instance)
        })
        .expect("Failed to open GPUI window");
    });
}
