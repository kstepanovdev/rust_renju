#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::NativeOptions;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

use eframe::{egui::{CentralPanel, Grid, ImageButton, TextureHandle, ColorImage}, epi::App, run_native};

mod field;
use field::Renju;

impl App for Renju {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let texture: &TextureHandle = &ui.ctx().load_texture("example", ColorImage::example());
            let img_size = 20.0 * texture.size_vec2() / texture.size_vec2().y;

            Grid::new("1").show(ui, |ui| {
                for idx in 0..self.field.len() {
                    ui.add(ImageButton::new(texture, img_size));
                    if (idx + 1) % 15 == 0 {
                        ui.end_row()
                    }
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Rust Renju"
    }
}


fn main() {
    let app = Renju::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
