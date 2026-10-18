use crate::components::*;
use crate::theme::HydraTheme;
use gpui::{prelude::*, *};
use shield_core::models::{Game, TorrentStatus, TorrentTask};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryFilter {
    All,
    Installed,
    Favorites,
}

pub fn render_catalogue(games: &[Game]) -> Div {
    let theme = HydraTheme::default();
    let featured_game = games.first();

    div()
        .flex()
        .flex_col()
        .w_full()
        .h_full()
        .overflow_y_hidden()
        .p_8()
        .gap_8()
        // Hero Featured Game Banner
        .when_some(featured_game, |this, game| {
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .justify_end()
                    .w_full()
                    .h(px(320.0))
                    .rounded_xl()
                    .p_8()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.bg_card)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .max_w(px(520.0))
                            .child(
                                div()
                                    .text_color(theme.brand_teal)
                                    .text_size(AbsoluteLength::Pixels(px(12.0)))
                                    .font_weight(FontWeight::BOLD)
                                    .child("FEATURED TITLE")
                            )
                            .child(
                                div()
                                    .text_color(theme.text_muted)
                                    .text_size(AbsoluteLength::Pixels(px(28.0)))
                                    .font_weight(FontWeight::BLACK)
                                    .child(game.title.clone())
                            )
                            .child(
                                div()
                                    .text_color(theme.text_body)
                                    .text_size(AbsoluteLength::Pixels(px(13.5)))
                                    .child("Explore top community recommendations, high speed downloads and instant Proton launcher support.")
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .mt_2()
                                    .child(render_button("Download & Play", ButtonVariant::Primary))
                                    .child(render_button("View Details", ButtonVariant::Secondary))
                            )
                    )
            )
        })
        // Catalogue Grid Section
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .text_size(AbsoluteLength::Pixels(px(18.0)))
                                .font_weight(FontWeight::BOLD)
                                .child("Trending in Catalogue")
                        )
                        .child(
                            div()
                                .text_color(theme.brand_teal)
                                .text_size(AbsoluteLength::Pixels(px(12.5)))
                                .cursor_pointer()
                                .child("View All →")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_4()
                        .children(games.iter().map(|g| {
                            render_card(
                                &g.title,
                                g.platform.as_str(),
                                g.executable_path.is_some(),
                                Some(g.playtime_seconds as f32 / 3600.0),
                            )
                        }))
                )
        )
}

pub fn render_library(games: &[Game], filter: LibraryFilter) -> Div {
    let theme = HydraTheme::default();

    let filtered_games: Vec<&Game> = games
        .iter()
        .filter(|g| match filter {
            LibraryFilter::All => true,
            LibraryFilter::Installed => g.executable_path.is_some(),
            LibraryFilter::Favorites => g.favorite,
        })
        .collect();

    div()
        .flex()
        .flex_col()
        .w_full()
        .h_full()
        .overflow_y_hidden()
        .p_8()
        .gap_6()
        // Header Bar & Filters
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .w_full()
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .gap_3()
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .text_size(AbsoluteLength::Pixels(px(22.0)))
                                .font_weight(FontWeight::BOLD)
                                .child("My Library")
                        )
                        .child(
                            div()
                                .text_color(theme.text_secondary)
                                .text_size(AbsoluteLength::Pixels(px(13.0)))
                                .child(format!("{} titles", filtered_games.len()))
                        )
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_3()
                                .py_1p5()
                                .rounded_md()
                                .bg(if filter == LibraryFilter::All { theme.brand_teal } else { theme.bg_card })
                                .text_color(if filter == LibraryFilter::All { rgba(0x000000ff).into() } else { theme.text_body })
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_size(AbsoluteLength::Pixels(px(12.0)))
                                .child("All")
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1p5()
                                .rounded_md()
                                .bg(if filter == LibraryFilter::Installed { theme.brand_teal } else { theme.bg_card })
                                .text_color(if filter == LibraryFilter::Installed { rgba(0x000000ff).into() } else { theme.text_body })
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_size(AbsoluteLength::Pixels(px(12.0)))
                                .child("Installed")
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1p5()
                                .rounded_md()
                                .bg(if filter == LibraryFilter::Favorites { theme.brand_teal } else { theme.bg_card })
                                .text_color(if filter == LibraryFilter::Favorites { rgba(0x000000ff).into() } else { theme.text_body })
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_size(AbsoluteLength::Pixels(px(12.0)))
                                .child("Favorites")
                        )
                )
        )
        // Library Grid
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_4()
                .children(filtered_games.into_iter().map(|g| {
                    render_card(
                        &g.title,
                        g.platform.as_str(),
                        g.executable_path.is_some(),
                        Some(g.playtime_seconds as f32 / 3600.0),
                    )
                }))
        )
}

pub fn render_downloads(active_tasks: &[TorrentTask]) -> Div {
    let theme = HydraTheme::default();

    div()
        .flex()
        .flex_col()
        .w_full()
        .h_full()
        .overflow_y_hidden()
        .p_8()
        .gap_6()
        // Title Bar
        .child(
            div()
                .flex()
                .items_baseline()
                .gap_3()
                .child(
                    div()
                        .text_color(theme.text_muted)
                        .text_size(AbsoluteLength::Pixels(px(22.0)))
                        .font_weight(FontWeight::BOLD)
                        .child("Downloads Manager")
                )
                .child(
                    div()
                        .text_color(theme.text_secondary)
                        .text_size(AbsoluteLength::Pixels(px(13.0)))
                        .child(format!("{} active", active_tasks.len()))
                )
        )
        // Active Download Cards
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .children(active_tasks.iter().map(|task| {
                    let progress_pct = (task.progress * 100.0) as u32;
                    let downloaded_gb = (task.downloaded_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
                    let total_gb = (task.total_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
                    let speed_mb = (task.download_speed_bps as f64) / (1024.0 * 1024.0);
                    let progress_width = px(task.progress.clamp(0.0, 1.0) * 600.0);

                    div()
                        .flex()
                        .flex_col()
                        .p_5()
                        .rounded_xl()
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.bg_card)
                        .gap_4()
                        // Task Header
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(
                                            div()
                                                .text_color(theme.text_muted)
                                                .font_weight(FontWeight::BOLD)
                                                .text_size(AbsoluteLength::Pixels(px(15.0)))
                                                .child(task.name.clone())
                                        )
                                        .child(
                                            match task.status {
                                                TorrentStatus::Downloading => render_badge("DOWNLOADING", BadgeVariant::Brand),
                                                TorrentStatus::Paused => render_badge("PAUSED", BadgeVariant::Warning),
                                                TorrentStatus::Completed => render_badge("COMPLETED", BadgeVariant::Success),
                                                TorrentStatus::Seeding => render_badge("SEEDING", BadgeVariant::Info),
                                                TorrentStatus::Queued => render_badge("QUEUED", BadgeVariant::Default),
                                                TorrentStatus::Error => render_badge("ERROR", BadgeVariant::Default),
                                            }
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(render_button("Pause", ButtonVariant::Secondary))
                                        .child(render_button("Cancel", ButtonVariant::Outline))
                                )
                        )
                        // Progress Bar
                        .child(
                            div()
                                .w_full()
                                .h(px(8.0))
                                .rounded_full()
                                .bg(theme.bg_dark)
                                .overflow_hidden()
                                .child(
                                    div()
                                        .h_full()
                                        .w(progress_width)
                                        .bg(theme.brand_teal)
                                        .rounded_full()
                                )
                        )
                        // Metrics Footer
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .text_color(theme.text_secondary)
                                .text_size(AbsoluteLength::Pixels(px(12.0)))
                                .child(
                                    div()
                                        .child(format!("{:.2} GB / {:.2} GB ({}%)", downloaded_gb, total_gb, progress_pct))
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_4()
                                        .child(format!("↓ {:.1} MB/s", speed_mb))
                                        .child(format!("Status: {}", task.status.as_str()))
                                )
                        )
                }))
        )
}

pub fn render_game_detail(game: &Game) -> Div {
    let theme = HydraTheme::default();
    let is_installed = game.executable_path.is_some();

    div()
        .flex()
        .flex_col()
        .w_full()
        .h_full()
        .overflow_y_hidden()
        // Hero Trailer Video & Backdrop Container (height 380px)
        .child(
            div()
                .relative()
                .w_full()
                .h(px(380.0))
                .bg(theme.bg_dark)
                .border_b_1()
                .border_color(theme.border)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .w(px(56.0))
                                .h(px(56.0))
                                .rounded_full()
                                .bg(theme.brand_teal)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgba(0x000000ff))
                                .font_weight(FontWeight::BLACK)
                                .text_size(AbsoluteLength::Pixels(px(20.0)))
                                .cursor_pointer()
                                .child("▶")
                        )
                        .child(
                            div()
                                .text_color(theme.text_secondary)
                                .text_size(AbsoluteLength::Pixels(px(12.5)))
                                .child("FFmpeg Hardware-Accelerated Video Trailer Preview")
                        )
                )
        )
        // Game Info & Actions Body
        .child(
            div()
                .flex()
                .p_8()
                .gap_8()
                // Left Poster Artwork (220x310)
                .child(
                    div()
                        .w(px(220.0))
                        .h(px(310.0))
                        .rounded_xl()
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.bg_card)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_color(theme.text_secondary)
                                .font_weight(FontWeight::BOLD)
                                .child("Poster Art")
                        )
                )
                // Right Metadata & Launch Controls
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .flex_1()
                        // Title & Platform Badges
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_color(theme.text_muted)
                                        .text_size(AbsoluteLength::Pixels(px(28.0)))
                                        .font_weight(FontWeight::BLACK)
                                        .child(game.title.clone())
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(render_badge(game.platform.as_str(), BadgeVariant::Brand))
                                        .when(is_installed, |this| {
                                            this.child(render_badge("READY TO PLAY", BadgeVariant::Success))
                                        })
                                )
                        )
                        // Action Buttons Bar
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_4()
                                .py_2()
                                .child(
                                    if is_installed {
                                        render_button("▶ PLAY NOW", ButtonVariant::Primary)
                                    } else {
                                        render_button("⬇ DOWNLOAD GAME", ButtonVariant::Primary)
                                    }
                                )
                                .child(render_button("Cloud Save Sync", ButtonVariant::Secondary))
                                .child(render_button("Settings & Wine", ButtonVariant::Outline))
                        )
                        // Playtime & Stats
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_6()
                                .py_2()
                                .border_y_1()
                                .border_color(theme.border)
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_color(theme.text_secondary)
                                                .text_size(AbsoluteLength::Pixels(px(11.0)))
                                                .child("TIME PLAYED")
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.text_muted)
                                                .font_weight(FontWeight::BOLD)
                                                .text_size(AbsoluteLength::Pixels(px(14.0)))
                                                .child(format!("{:.1} hours", game.playtime_seconds as f32 / 3600.0))
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_color(theme.text_secondary)
                                                .text_size(AbsoluteLength::Pixels(px(11.0)))
                                                .child("ACHIEVEMENTS")
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.text_muted)
                                                .font_weight(FontWeight::BOLD)
                                                .text_size(AbsoluteLength::Pixels(px(14.0)))
                                                .child("RetroAchievements Enabled")
                                        )
                                )
                        )
                        // Description Paragraph
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_color(theme.text_muted)
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(AbsoluteLength::Pixels(px(14.0)))
                                        .child("About This Game")
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.text_body)
                                            .text_size(AbsoluteLength::Pixels(px(13.0)))
                                            .child("High fidelity native Linux & Windows gaming launcher built with pure Rust and hardware-accelerated GPU graphics.")
                                    )
                            )
                    )
            )
}

pub fn render_settings() -> Div {
    let theme = HydraTheme::default();

    let render_row = |title: &'static str, description: &'static str, badge: Option<&'static str>| {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .rounded_lg()
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_color(theme.text_muted)
                            .font_weight(FontWeight::BOLD)
                            .text_size(AbsoluteLength::Pixels(px(14.0)))
                            .child(title)
                    )
                    .child(
                        div()
                            .text_color(theme.text_secondary)
                            .text_size(AbsoluteLength::Pixels(px(12.5)))
                            .child(description)
                    )
            )
            .when_some(badge, |this, b| {
                this.child(render_badge(b, BadgeVariant::Brand))
            })
    };

    div()
        .flex()
        .flex_col()
        .w_full()
        .h_full()
        .overflow_y_hidden()
        .p_8()
        .gap_6()
        .child(
            div()
                .text_color(theme.text_muted)
                .text_size(AbsoluteLength::Pixels(px(22.0)))
                .font_weight(FontWeight::BOLD)
                .child("Application Settings")
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(render_row("Renderer Backend", "Pure Rust native GPU acceleration (Vulkan / Blade)", Some("ACTIVE")))
                .child(render_row("Media Decoders", "Bundled FFmpeg hardware-accelerated video codecs", Some("BUNDLED")))
                .child(render_row("P2P BitTorrent Client", "Embedded librqbit multi-threaded engine", Some("READY")))
                .child(render_row("Proton & Wine Supervisor", "Auto-detection of Steam Proton, GE-Proton and system Wine", Some("LINUX")))
                .child(render_row("Cloud Save Synchronization", "Direct Ludusavi save manifest extraction and S3 backup", Some("ENABLED")))
        )
}
