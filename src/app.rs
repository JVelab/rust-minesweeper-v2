use crate::game::{Game, GameState};
use crate::settings::Settings;
use crate::theme::Theme;
use crate::ui::CellPainter;
use egui::{Align2, FontId, RichText, Vec2, Window};

pub struct MinesweeperApp {
    game: Game,
    settings: Settings,
    theme: Theme,
    cell_painter: CellPainter,
    cell_size: f32,
    show_settings: bool,
    show_game_over_dialog: bool,
    custom_width: usize,
    custom_height: usize,
    custom_mines: usize,
}

impl MinesweeperApp {
    pub fn new() -> Self {
        let default_settings = Settings::default();
        Self {
            game: Game::from_difficulty(default_settings.difficulty),
            settings: default_settings,
            theme: Theme::dark(),
            cell_painter: CellPainter::new(),
            cell_size: 36.0,
            show_settings: false,
            show_game_over_dialog: false,
            custom_width: 16,
            custom_height: 16,
            custom_mines: 40,
        }
    }

    fn reset_game(&mut self) {
        let (w, h, m) = self.settings.difficulty.dimensions();
        self.game.reset(w, h, m);
        self.show_game_over_dialog = false;
    }

    fn start_new_game(&mut self, difficulty: crate::settings::Difficulty) {
        self.settings.difficulty = difficulty;
        self.game.reset_with_difficulty(difficulty);
        self.show_settings = false;
        self.show_game_over_dialog = false;
    }
}

impl eframe::App for MinesweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.game.update_time();
        
        if self.game.state == GameState::Won || self.game.state == GameState::Lost {
            self.show_game_over_dialog = true;
        }

        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::default().fill(self.theme.colors.surface).inner_margin(8.0))
            .show(ctx, |ui| {
                self.render_header(ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.colors.background).outer_margin(0.0))
            .show(ctx, |ui| {
                ui.set_height(ui.available_height());
                self.render_board_anchored(ui);
            });

        if self.show_settings {
            self.render_settings_dialog(ctx);
        }

        if self.show_game_over_dialog {
            self.render_game_over_dialog(ctx);
        }
    }
}

impl MinesweeperApp {

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let theme_btn = egui::Button::new(if self.theme.is_dark { "☀️" } else { "🌙" })
                .frame(true)
                .fill(self.theme.colors.surface);
            if ui.add(theme_btn).clicked() {
                self.theme.toggle();
            }
            
            ui.add_space(16.0);
            
            let diff_name = self.settings.difficulty.display_name();
            let diff_btn = egui::Button::new(format!("Difficulty: {}", diff_name))
                .frame(true)
                .fill(self.theme.colors.surface_elevated);
            if ui.add(diff_btn).clicked() {
                self.show_settings = true;
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let reset_btn = egui::Button::new("🔄 New Game")
                    .frame(true)
                    .fill(self.theme.colors.surface_elevated)
                    .rounding(8.0);
                if ui.add(reset_btn).clicked() {
                    self.reset_game();
                }
                
                ui.add_space(16.0);
                
                let mines_text = RichText::new(format!("💣 {}", self.game.get_mines_remaining()))
                    .font(FontId::proportional(24.0))
                    .color(self.theme.colors.text);
                ui.label(mines_text);
                
                ui.add_space(16.0);
                
                let time_text = RichText::new(self.game.format_time())
                    .font(FontId::proportional(24.0))
                    .color(self.theme.colors.text);
                ui.label(time_text);
            });
        });
    }

    fn render_board_anchored(&mut self, ui: &mut egui::Ui) {
        let (w, h, _) = self.settings.difficulty.dimensions();
        let available = ui.available_width();
        
        let max_cell_size = (available - 32.0) / w as f32;
        let cell_size = self.cell_size.min(max_cell_size).max(20.0);
        
        let board_width = cell_size * w as f32 + (w - 1) as f32 * 2.0;
        let board_height = cell_size * h as f32 + (h - 1) as f32 * 2.0;
        
        let board_rect = egui::Rect::from_min_size(
            egui::pos2(
                (ui.available_width() - board_width) / 2.0,
                ui.available_rect_before_wrap().min.y
            ),
            Vec2::new(board_width, board_height),
        );
        
        let board_area_rect = egui::Rect::from_min_size(
            board_rect.min,
            Vec2::new(board_width, board_height),
        );
        
        let response = ui.allocate_rect(board_area_rect, egui::Sense::click());
        
        if let Some(cursor_pos) = ui.input(|i| i.pointer.interact_pos()) {
            if board_area_rect.contains(cursor_pos) {
                let local_pos = egui::pos2(cursor_pos.x - board_area_rect.min.x, cursor_pos.y - board_area_rect.min.y);
                let col = (local_pos.x / (cell_size + 2.0)) as usize;
                let row = (local_pos.y / (cell_size + 2.0)) as usize;
                self.cell_painter.set_hover(Some((row, col)));
            } else {
                self.cell_painter.set_hover(None);
            }
        }
        
        let was_game_over = self.game.is_game_over();
        
        if response.clicked() && !was_game_over {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if board_area_rect.contains(pos) {
                    let local_pos = egui::pos2(pos.x - board_area_rect.min.x, pos.y - board_area_rect.min.y);
                    let col = (local_pos.x / (cell_size + 2.0)) as usize;
                    let row = (local_pos.y / (cell_size + 2.0)) as usize;
                    
                    if row < h && col < w {
                        let exploded = self.game.reveal(row, col);
                        if exploded {
                            self.game.state = GameState::Lost;
                            self.game.board.reveal_all_mines();
                        } else {
                            self.game.check_game_end();
                        }
                    }
                }
            }
        }
        
        if response.secondary_clicked() && !was_game_over {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if board_area_rect.contains(pos) {
                    let local_pos = egui::pos2(pos.x - board_area_rect.min.x, pos.y - board_area_rect.min.y);
                    let col = (local_pos.x / (cell_size + 2.0)) as usize;
                    let row = (local_pos.y / (cell_size + 2.0)) as usize;
                    
                    if row < h && col < w {
                        self.game.flag(row, col);
                    }
                }
            }
        }
        
        let painter = ui.painter();
        
        for row in 0..h {
            for col in 0..w {
                let cell_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        board_rect.min.x + col as f32 * (cell_size + 2.0),
                        board_rect.min.y + row as f32 * (cell_size + 2.0),
                    ),
                    Vec2::splat(cell_size),
                );
                
                if let Some(cell) = self.game.board.get_cell(row, col) {
                    let is_hovered = self.cell_painter.is_hovered(row, col);
                    
                    let (bg_color, stroke) = match cell.state {
                        crate::cell::CellState::Hidden | crate::cell::CellState::Flagged | crate::cell::CellState::Questioned => {
                            let base = if is_hovered { self.theme.colors.cell_hover } else { self.theme.colors.cell_hidden };
                            (base, egui::Stroke::new(1.0, Self::darken(base, 0.1)))
                        }
                        crate::cell::CellState::Revealed => {
                            if cell.is_mine {
                                (self.theme.colors.accent_lose, egui::Stroke::new(1.0, self.theme.colors.mine_color))
                            } else {
                                (self.theme.colors.cell_revealed, egui::Stroke::new(0.0, egui::Color32::TRANSPARENT))
                            }
                        }
                    };
                    
                    painter.rect_filled(cell_rect, egui::Rounding::same(4.0), bg_color);
                    if stroke.width > 0.0 {
                        painter.rect_stroke(cell_rect, egui::Rounding::same(4.0), stroke);
                    }
                    
                    if cell.is_exploded {
                        painter.circle_filled(cell_rect.center(), cell_size * 0.35, self.theme.colors.accent_lose);
                    } else if matches!(cell.state, crate::cell::CellState::Revealed) && cell.is_mine {
                        painter.circle_filled(cell_rect.center(), cell_size * 0.3, self.theme.colors.mine_color);
                    } else if cell.is_flagged() {
                        Self::paint_flag(painter, cell_rect, self.theme.colors.flag_color);
                    } else if matches!(cell.state, crate::cell::CellState::Revealed) && cell.adjacent_mines > 0 {
                        Self::paint_number(painter, cell_rect, cell.adjacent_mines);
                    } else if matches!(cell.state, crate::cell::CellState::Questioned) {
                        Self::paint_question(painter, cell_rect, &self.theme.colors);
                    }
                }
            }
        }
    }

    fn darken(color: egui::Color32, amount: f32) -> egui::Color32 {
        let (r, g, b, a) = (color.r(), color.g(), color.b(), color.a());
        let factor = 1.0 - amount;
        egui::Color32::from_rgba_unmultiplied(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
            a,
        )
    }

    fn paint_flag(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
        let center = rect.center();
        let s = rect.height() * 0.5;
        
        let tri_points = vec![
            egui::pos2(center.x - s * 0.4, center.y - s * 0.5),
            egui::pos2(center.x - s * 0.4, center.y + s * 0.3),
            egui::pos2(center.x + s * 0.4, center.y),
        ];
        
        painter.add(egui::Shape::convex_polygon(tri_points, color, egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)));
        
        let stem_start = egui::pos2(center.x - s * 0.4, center.y + s * 0.3);
        let stem_end = egui::pos2(center.x - s * 0.4, center.y + s * 0.5);
        painter.line_segment([stem_start, stem_end], (2.0, color));
    }

    fn paint_number(painter: &egui::Painter, rect: egui::Rect, num: u8) {
        let color = crate::theme::ThemeColors::number_color(num);
        let text = format!("{}", num);
        let font = FontId::proportional(rect.height() * 0.5);
        
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            color,
        );
    }

    fn paint_question(painter: &egui::Painter, rect: egui::Rect, theme: &crate::theme::ThemeColors) {
        let text = "?";
        let font = FontId::proportional(rect.height() * 0.5);
        
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            theme.text_secondary,
        );
    }

    fn render_settings_dialog(&mut self, ctx: &egui::Context) {
        let mut open = true;
        Window::new("Settings")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_width(320.0);
                
                ui.label(RichText::new("New Game").font(FontId::proportional(20.0)));
                ui.add_space(16.0);
                
                if ui.button("Easy (9x9, 10 mines)").clicked() {
                    self.start_new_game(crate::settings::Difficulty::Easy);
                }
                if ui.button("Medium (16x16, 40 mines)").clicked() {
                    self.start_new_game(crate::settings::Difficulty::Medium);
                }
                if ui.button("Hard (30x16, 99 mines)").clicked() {
                    self.start_new_game(crate::settings::Difficulty::Hard);
                }
                
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);
                
                ui.label("Custom Difficulty:");
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(egui::DragValue::new(&mut self.custom_width).range(5..=50));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.add(egui::DragValue::new(&mut self.custom_height).range(5..=50));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Mines:");
                    let max_mines = self.custom_width * self.custom_height - 1;
                    ui.add(egui::DragValue::new(&mut self.custom_mines).range(1..=max_mines));
                });
                
                if ui.button("Start Custom Game").clicked() {
                    let diff = crate::settings::Difficulty::Custom {
                        width: self.custom_width,
                        height: self.custom_height,
                        mines: self.custom_mines,
                    };
                    self.start_new_game(diff);
                }
                
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);
                
                ui.checkbox(&mut self.settings.animations_enabled, "Enable animations");
                ui.checkbox(&mut self.settings.sound_enabled, "Enable sounds");
                ui.add_space(16.0);
                
                if ui.button("Close").clicked() {
                    self.show_settings = false;
                }
            });
        
        if !open {
            self.show_settings = false;
        }
    }

    fn render_game_over_dialog(&mut self, ctx: &egui::Context) {
        let is_won = self.game.state == GameState::Won;
        let title = if is_won { "Victory!" } else { "Game Over" };
        let message = if is_won {
            format!("Congratulations! Time: {}", self.game.format_time())
        } else {
            "You hit a mine!".to_string()
        };
        
        let mut open = true;
        Window::new(title)
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_width(280.0);
                ui.add_space(12.0);
                ui.label(RichText::new(message).font(FontId::proportional(16.0)));
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Play Again").clicked() {
                        self.reset_game();
                    }
                    if ui.button("New Game").clicked() {
                        self.show_settings = true;
                    }
                });
            });
        
        if !open {
            self.show_game_over_dialog = false;
        }
    }
}