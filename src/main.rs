#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::{
    egui::{Label, Window},
    NativeOptions,
};
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
use eframe::{
    egui::{menu, Button, CentralPanel, Layout, TopBottomPanel},
    epi::App,
    run_native,
};

mod field;
use field::Renju;
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
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        ctx.request_repaint();

        if self.connected {
            self.handle_game_action();

            render_control_panel(ctx, frame);
            CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.);
                    self.render_field(ui);
                    if ui.add(Button::new("Start a new game")).clicked() {
                        self.reset();
                    }
                });

                match &self.winner {
                    Some(winner) => {
                        render_popup(ctx, winner.clone());
                    }
                    None => {}
                }
            });
        } else {
            self.render_connection_panel(ctx);
        }
    }

    fn name(&self) -> &str {
        "Rust Renju"
    }
}

fn render_popup(ctx: &eframe::egui::Context, winner: String) {
    Window::new("Winner has been found!")
        // .anchor(Align2::CENTER_CENTER, (0., 0.))
        .show(ctx, |ui| {
            ui.add(Label::new(winner));
        });
}

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
