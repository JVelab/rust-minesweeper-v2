mod app;
mod board;
mod cell;
mod game;
mod settings;
mod theme;
mod ui;

use eframe::egui;
use app::MinesweeperApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_min_inner_size([1050.0, 750.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Ok(Box::new(MinesweeperApp::new()))),
    )
}