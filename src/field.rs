use std::{
    io::Write,
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
    thread,
};
#[derive(Serialize, Deserialize)]
enum GameAction {
    Connect(String),
    Move(usize, String),
    Reset,
}

use eframe::egui::{self, Color32, ColorImage, Grid, ImageButton, TextureHandle, Ui, Window};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerResponse {
    Ok,
    Fail,
    Move(usize, usize),
}

pub struct Renju {
    pub field: [usize; 225],
    pub enabled: bool,
    pub config: RenjuConfig,
    pub stream: Option<TcpStream>,
    pub rx: Option<Receiver<ServerResponse>>,
    pub tx: Option<Sender<ServerResponse>>,
}

impl Default for Renju {
    fn default() -> Self {
        let config: RenjuConfig = confy::load("renju").unwrap_or_default();
        Self {
            field: [0; 225],
            enabled: true,
            config,
            stream: None,
            tx: None,
            rx: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct RenjuConfig {
    pub dark_mode: bool,
    pub connection_ip: String,
    pub username: String,
}

impl Renju {
    pub fn new() -> Self {
        let config: RenjuConfig = confy::load("renju").unwrap_or_default();
        Renju {
            field: [0; 225],
            enabled: true,
            config,
            stream: None,
            tx: None,
            rx: None,
        }
    }

    pub fn render_connection_panel(&mut self, ctx: &eframe::egui::Context) {
        Window::new("Connection panel").show(ctx, |ui| {
            ui.label("Enter game server's IP:");
            let _ip_info_field = ui.text_edit_singleline(&mut self.config.connection_ip);
            ui.label("Enter a username:");
            let _username_field = ui.text_edit_singleline(&mut self.config.username);

            if ui.input().key_pressed(egui::Key::Enter) {
                match confy::store(
                    "renju",
                    RenjuConfig {
                        dark_mode: self.config.dark_mode,
                        connection_ip: self.config.connection_ip.clone(),
                        username: self.config.username.clone(),
                    },
                ) {
                    Ok(_) => tracing::error!("all green"),
                    Err(e) => tracing::error!("Failed saving app state: {}", e),
                }
            }
        });
    }

    pub fn render_field(&mut self, ui: &mut Ui) {
        let colors = vec![
            Color32::TRANSPARENT,
            Color32::LIGHT_RED,
            Color32::LIGHT_BLUE,
        ];
        Grid::new("field").spacing([5.0, 5.0]).show(ui, |ui| {
            ui.set_enabled(self.enabled);

            for (idx, cell_color) in self.field.into_iter().enumerate() {
                let texture: &TextureHandle = &ui
                    .ctx()
                    .load_texture("cell", ColorImage::new([80, 80], colors[cell_color]));
                let img_size = 50.0 * texture.size_vec2() / texture.size_vec2().y;
                let cell = ui.add(ImageButton::new(texture, img_size));

                if cell.clicked() {
                    let data =
                        bincode::serialize(&GameAction::Move(idx, self.config.username.clone()))
                            .unwrap();
                    self.stream.as_mut().unwrap().write_all(&data).unwrap();
                }
                if (idx + 1) % 15 == 0 {
                    ui.end_row()
                }
            }
        });
    }

    pub fn update_field(&mut self) {
        let rx = self.rx.as_mut().unwrap();
        match rx.try_recv() {
            Ok(response) => match response {
                ServerResponse::Ok => {
                    tracing::warn!("ok rx side")
                }
                ServerResponse::Fail => {}
                ServerResponse::Move(move_id, color) => {
                    tracing::warn!("move_id: {}, color: {}", move_id, color);
                    self.field[move_id] = color;
                }
            },
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }
}
