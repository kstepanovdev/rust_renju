#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::{egui::Ui, NativeOptions};
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
use eframe::{
    egui::{menu, Button, CentralPanel, Label, Layout, TopBottomPanel, Window},
    epi::App,
    run_native,
};

mod field;
use field::{Player, Renju};

impl App for Renju {
    fn setup(
        &mut self,
        _ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        render_control_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.);
            self.render_field(ui);
            if ui.add(Button::new("Start a new game")).clicked() {
                self.reset_game();
            }

            match &self.winner {
                Some(player) => render_popup(ctx, &player),
                None => {}
            }
        });
    }

    fn name(&self) -> &str {
        "Rust Renju"
    }
}

fn render_popup(ctx: &eframe::egui::Context, player: &Player) {
    Window::new("Winner has been found!").show(ctx, |ui| match player {
        &Player::One => ui.add(Label::new("Player One has won")),
        &Player::Two => ui.add(Label::new("Player Two has won")),
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
    let app = Renju::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
