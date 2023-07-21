#![deny(clippy::pedantic)]
// 在 release 中隐藏控制台
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui::vec2, run_native, NativeOptions};

use ui::App;

mod config;
mod logic;
mod ui;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let native_options =
        NativeOptions { initial_window_size: Some(vec2(540.0, 1080.0)), ..Default::default() };

    run_native("子弹姬", native_options, Box::new(|cc| Box::new(App::new(cc))))
}
