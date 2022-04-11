#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::NativeOptions;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
use eframe::{
    egui::{CentralPanel, Label, TopBottomPanel, Window},
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
        CentralPanel::default().show(ctx, |ui| {
            self.render_field(ui);

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

fn main() {
    let app = Renju::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
