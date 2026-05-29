use crate::cell::CellState;
use crate::theme::ThemeColors;
use egui::{Color32, FontId, Rounding, Sense, Stroke, Vec2};

pub struct CellPainter {
    hover_cell: Option<(usize, usize)>,
}

impl CellPainter {
    pub fn new() -> Self {
        Self { hover_cell: None }
    }

    pub fn set_hover(&mut self, cell: Option<(usize, usize)>) {
        self.hover_cell = cell;
    }

    pub fn is_hovered(&self, row: usize, col: usize) -> bool {
        self.hover_cell == Some((row, col))
    }

    pub fn paint_cell_at(
        &self,
        ui: &mut egui::Ui,
        cell: &crate::cell::Cell,
        pos: egui::Pos2,
        size: f32,
        theme: &ThemeColors,
    ) -> egui::Response {
        let rect = egui::Rect::from_min_size(pos, Vec2::splat(size));
        
        let is_hovered = self.hover_cell.map(|(r, c)| {
            let cell_pos = self.cell_to_pixel(r, c, size);
            rect.contains(cell_pos)
        }).unwrap_or(false);
        
        let (bg_color, stroke) = match cell.state {
            CellState::Hidden | CellState::Flagged | CellState::Questioned => {
                let base = if is_hovered { theme.cell_hover } else { theme.cell_hidden };
                (base, Stroke::new(1.0, Self::darken(base, 0.1)))
            }
            CellState::Revealed => {
                if cell.is_mine {
                    (theme.accent_lose, Stroke::new(1.0, theme.mine_color))
                } else {
                    (theme.cell_revealed, Stroke::new(0.0, Color32::TRANSPARENT))
                }
            }
        };
        
        ui.painter().rect_filled(rect, Rounding::same(4.0), bg_color);
        if stroke.width > 0.0 {
            ui.painter().rect_stroke(rect, Rounding::same(4.0), stroke);
        }
        
        if cell.is_exploded {
            ui.painter().circle_filled(rect.center(), size * 0.35, theme.accent_lose);
        } else if matches!(cell.state, CellState::Revealed) && cell.is_mine {
            ui.painter().circle_filled(rect.center(), size * 0.3, theme.mine_color);
        } else if cell.is_flagged() {
            self.paint_flag(ui, rect, &theme.flag_color);
        } else if matches!(cell.state, CellState::Revealed) && cell.adjacent_mines > 0 {
            self.paint_number(ui, rect, cell.adjacent_mines);
        } else if matches!(cell.state, CellState::Questioned) {
            self.paint_question(ui, rect, theme);
        }
        
        ui.interact(rect, egui::Id::new(cell as *const _ as usize), Sense::click())
    }

    fn cell_to_pixel(&self, row: usize, col: usize, cell_size: f32) -> egui::Pos2 {
        egui::Pos2::new(col as f32 * cell_size, row as f32 * cell_size)
    }

    fn paint_flag(&self, ui: &mut egui::Ui, rect: egui::Rect, color: &Color32) {
        let center = rect.center();
        let s = rect.height() * 0.5;
        
        let tri_points = vec![
            egui::pos2(center.x - s * 0.4, center.y - s * 0.5),
            egui::pos2(center.x - s * 0.4, center.y + s * 0.3),
            egui::pos2(center.x + s * 0.4, center.y),
        ];
        
        ui.painter().add(egui::Shape::convex_polygon(tri_points, *color, Stroke::new(0.0, Color32::TRANSPARENT)));
        
        let stem_start = egui::pos2(center.x - s * 0.4, center.y + s * 0.3);
        let stem_end = egui::pos2(center.x - s * 0.4, center.y + s * 0.5);
        ui.painter().line_segment([stem_start, stem_end], (2.0, *color));
    }

    fn paint_number(&self, ui: &mut egui::Ui, rect: egui::Rect, num: u8) {
        let color = ThemeColors::number_color(num);
        let text = format!("{}", num);
        let font = FontId::proportional(rect.height() * 0.5);
        
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            color,
        );
    }

    fn paint_question(&self, ui: &mut egui::Ui, rect: egui::Rect, theme: &ThemeColors) {
        let text = "?";
        let font = FontId::proportional(rect.height() * 0.5);
        
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            theme.text_secondary,
        );
    }

    fn darken(color: Color32, amount: f32) -> Color32 {
        let (r, g, b, a) = (color.r(), color.g(), color.b(), color.a());
        let factor = 1.0 - amount;
        Color32::from_rgba_unmultiplied(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
            a,
        )
    }
}

impl Default for CellPainter {
    fn default() -> Self {
        Self::new()
    }
}