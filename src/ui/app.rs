use std::fs::read;

use eframe::{
    egui::{
        CentralPanel, Context, FontData, FontDefinitions, FontFamily, FontId, RichText, TextEdit,
        Ui,
    },
    CreationContext, Frame,
};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, config: Config) -> Self {
        Self::configure_fonts(&cc.egui_ctx);
        Self { config }
    }

    /// 配置字体
    fn configure_fonts(ctx: &Context) -> Option<()> {
        let fonts = Config::fonts();
        if !fonts.is_empty() {
            let font_file = fonts.first()?;
            let font_name = font_file.file_stem()?.to_str()?;
            let font_file_bytes = read(font_file).ok()?;

            let font_data = FontData::from_owned(font_file_bytes);
            let mut font_def = FontDefinitions::default();
            font_def.font_data.insert(font_name.to_string(), font_data);

            font_def
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.to_string());
            font_def.families.entry(FontFamily::Monospace).or_default().push(font_name.to_string());

            ctx.set_fonts(font_def);
        }
        Some(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            render_address(&mut self.config, ui);
        });
    }
}

pub fn render_address(config: &mut Config, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let live_id = ui.label(
            RichText::new("直播间：https://live.bilibili.com/").font(FontId::proportional(18.0)),
        );
        let room_id = ui
            .add(
                TextEdit::singleline(&mut config.room_id)
                    .desired_width(90.0)
                    .font(FontId::proportional(18.0)),
            )
            .labelled_by(live_id.id);
        if room_id.lost_focus() {
            config.update();
        }
        if ui.button(RichText::new("连接").font(FontId::proportional(18.0))).clicked() {
            println!("连接");
        }
        if ui.button(RichText::new("断开").font(FontId::proportional(18.0))).clicked() {
            println!("断开");
        }
        if ui.button(RichText::new("测试").font(FontId::proportional(18.0))).clicked() {
            println!("测试");
        }
    });
}
