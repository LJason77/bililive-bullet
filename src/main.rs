#![deny(clippy::pedantic)]

use eframe::{egui::vec2, run_native, NativeOptions};

use config::Config;
use ui::App;

mod config;
mod logic;
mod ui;

fn main() -> eframe::Result<()> {
    // 读取配置
    let config = Config::read();
    println!("{config:?}");

    let native_options =
        NativeOptions { initial_window_size: Some(vec2(540.0, 1080.0)), ..Default::default() };

    run_native("子弹姬", native_options, Box::new(|cc| Box::new(App::new(cc, config))))
}
