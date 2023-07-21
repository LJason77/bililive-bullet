use std::{
    fs::read,
    sync::mpsc::{channel, Receiver, Sender},
};

use eframe::{
    egui::{
        style::TextStyle, Button, CentralPanel, Context, FontData, FontDefinitions, FontFamily,
        FontId, TextEdit, Ui,
    },
    CreationContext, Frame,
};

use crate::{config::Config, logic::DanmuInfo};

#[derive(PartialEq)]
pub enum State {
    Unconnected,
    Connecting,
    Connected,
}

pub struct StateChannel {
    pub state: State,
    pub tx: Sender<State>,
    pub rx: Receiver<State>,
}

impl StateChannel {
    pub fn send(&self, state: State) {
        self.tx.send(state).unwrap();
    }

    pub fn try_recv(&self) -> Option<State> {
        self.rx.try_recv().ok()
    }
}

pub struct App {
    pub room_id: String,
    pub config: Config,
    pub state: StateChannel,
}

impl App {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::configure_fonts(&cc.egui_ctx);
        let (tx, rx) = channel();
        let channel = StateChannel { state: State::Unconnected, tx, rx };
        let config = Config::read();
        Self { room_id: format!("{}", config.room_id), config, state: channel }
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
            if let Some(state) = self.state.try_recv() {
                self.state.state = state;
            }
            render_address(self, ui);
        });
    }
}

pub fn render_address(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.style_mut().override_font_id = Some(FontId::proportional(18.0));
        ui.style_mut().text_styles.insert(TextStyle::Button, FontId::proportional(18.0));

        let live_id = ui.label("直播间：https://live.bilibili.com/");
        ui.add(TextEdit::singleline(&mut app.room_id).desired_width(90.0)).labelled_by(live_id.id);

        let state = &app.state.state;
        if ui.add_enabled(state == &State::Unconnected, Button::new("连接")).clicked() {
            let sender = app.state.tx.clone();
            match app.room_id.parse::<u64>() {
                Ok(room_id) => {
                    tokio::spawn(async move {
                        sender.send(State::Connecting).unwrap();
                        match DanmuInfo::get_info(room_id).await {
                            Ok(info) => {
                                println!("{info:#?}");
                            }
                            Err(err) => {
                                sender.send(State::Unconnected).unwrap();
                                println!("{err:?}");
                            }
                        }
                    });
                }
                Err(_) => {
                    println!("错误");
                }
            }
        }
        if ui.add_enabled(state == &State::Connected, Button::new("断开")).clicked() {
            app.state.send(State::Unconnected);
            println!("断开");
        }
        if ui.button("测试").clicked() {
            println!("测试");
        }
    });
}
