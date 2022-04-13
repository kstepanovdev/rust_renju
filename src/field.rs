use eframe::egui::{Color32, ColorImage, Grid, ImageButton, TextureHandle, Ui};

pub struct Renju {
    pub field: [Color32; 225],
    pub turn: Player,
    pub winner: Option<Player>,
    pub enabled: bool,
}

#[derive(Clone, Copy)]
pub enum Player {
    One,
    Two,
}

impl Default for Renju {
    fn default() -> Self {
        Self {
            field: [Color32::TRANSPARENT; 225],
            turn: Player::One,
            winner: None,
            enabled: true,
        }
    }
}

impl Renju {
    pub fn new() -> Self {
        Renju {
            field: [Color32::TRANSPARENT; 225],
            turn: Player::One,
            winner: None,
            enabled: true,
        }
    }

    pub fn render_field(&mut self, ui: &mut Ui) {
        Grid::new("field").spacing([5.0, 5.0]).show(ui, |ui| {
            ui.set_enabled(self.enabled);
            for idx in 0..self.field.len() {
                let texture: &TextureHandle = &ui
                    .ctx()
                    .load_texture("example", ColorImage::new([80, 80], self.field[idx]));
                let img_size = 50.0 * texture.size_vec2() / texture.size_vec2().y;

                let cell = ui.add(ImageButton::new(texture, img_size));
                if cell.clicked() {
                    match (&self.turn, &self.field[idx]) {
                        (Player::One, &Color32::TRANSPARENT) => {
                            let current_color = Color32::LIGHT_RED;
                            self.field[idx] = current_color;
                            self.winner_check(current_color);
                            self.turn = Player::Two;
                        }
                        (Player::Two, &Color32::TRANSPARENT) => {
                            let current_color = Color32::LIGHT_BLUE;
                            self.field[idx] = current_color;
                            self.winner_check(current_color);
                            self.turn = Player::One;
                        }
                        (_, _) => {}
                    }
                }
                if (idx + 1) % 15 == 0 {
                    ui.end_row()
                }
            }
        });
    }

    fn winner_check(&mut self, win_color: Color32) {
        self.horizontal_check(win_color);
        [14, 15, 16].map(|shift| self.shift_check(win_color, shift));
    }

    fn shift_check(&mut self, win_color: Color32, shift: usize) {
        let mut idx = 0;
        let mut win_line = vec![];
        while idx < self.field.len() {
            if self.field[idx] != win_color {
                idx += 1;
                win_line = vec![];
                continue;
            }
            win_line.push(idx);
            let mut i = idx;
            while i + shift < self.field.len() && self.field[i + shift] == win_color {
                win_line.push(i);
                if win_line.len() >= 5 {
                    self.winner = Some(self.turn);
                    self.enabled = false;
                    return;
                }
                i += shift;
            }
            win_line = vec![];
            idx += 1;
        }
    }

    fn horizontal_check(&mut self, win_color: Color32) {
        let rows = self.field.chunks(15);
        for row in rows {
            let mut win_line = vec![];
            let mut idx = 0;
            while idx < row.len() {
                let cell_color = row[idx];
                if cell_color == win_color {
                    win_line.push(idx);
                } else {
                    win_line = vec![];
                }
                if win_line.len() >= 5 {
                    self.winner = Some(self.turn);
                    self.enabled = false;
                    return;
                }
                idx += 1;
            }
        }
    }
}
