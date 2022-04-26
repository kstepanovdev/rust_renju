#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
    sync::mpsc::channel,
    thread,
};

use eframe::NativeOptions;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
use eframe::{
    egui::{menu, Button, CentralPanel, Layout, TopBottomPanel},
    epi::App,
    run_native,
};

mod field;
use field::Renju;
use field::ServerResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum GameAction {
    Connect(String),
    Move(usize, String),
    Reset,
}

impl App for Renju {
    fn setup(
        &mut self,
        _ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        let (tx, rx) = channel();
        self.tx = Some(tx);
        self.rx = Some(rx);

        match TcpStream::connect(&self.config.connection_ip) {
            Ok(stream) => {
                self.stream = Some(stream);
                let data =
                    bincode::serialize(&GameAction::Connect(self.config.username.clone())).unwrap();
                self.stream.as_mut().unwrap().write_all(&data).unwrap();
            }
            Err(e) => panic!("{}", e),
        }

        let stream = self.stream.as_mut().unwrap().try_clone().unwrap();
        let mut reader = BufReader::new(stream);
        let tx = self.tx.as_mut().unwrap().clone();
        thread::spawn(move || loop {
            println!("keeeeeeek");
            let mut buffer = [0; 32];
            match reader.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        return;
                    };
                    let data = bincode::deserialize::<ServerResponse>(&buffer[..]).unwrap();
                    tracing::warn!("{:?}", data);
                    tx.send(data).unwrap();
                }
                Err(_) => {
                    println!("An error occurred, terminating connection with ",);
                    // stream.shutdown(Shutdown::Both).unwrap();
                }
            }
        });
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        self.update_field();

        self.render_connection_panel(ctx);

        render_control_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.);
                self.render_field(ui);
                if ui.add(Button::new("Start a new game")).clicked() {
                    *self = Renju::default();
                }
            });

            //     match &self.winner {
            //         Some(player) => {
            //             render_popup(ctx, player);
            //         }
            //         None => {}
            //     }
        });
    }

    fn name(&self) -> &str {
        "Rust Renju"
    }
}

// fn render_popup(ctx: &eframe::egui::Context, player: &Player) {
//     Window::new("Winner has been found!")
//         .anchor(Align2::CENTER_CENTER, vec2(0., 0.))
//         .show(ctx, |ui| match *player {
//             Player::One => ui.add(Label::new("Player One has won")),
//             Player::Two => ui.add(Label::new("Player Two has won")),
//         });
// }

fn render_control_panel(ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(10.);
        menu::bar(ui, |ui| {
            ui.with_layout(Layout::right_to_left(), |ui| {
                let close_btn = ui.add(Button::new("❌"));
                if close_btn.clicked() {
                    frame.quit();
                }
            });
        });
        ui.add_space(10.);
    });
}

fn main() {
    tracing_subscriber::fmt::init();
    let app = Renju::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
