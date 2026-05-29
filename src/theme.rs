use egui::Color32;

#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
    pub background: Color32,
    pub surface: Color32,
    pub surface_elevated: Color32,
    pub primary: Color32,
    pub primary_hover: Color32,
    pub secondary: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub cell_hidden: Color32,
    pub cell_revealed: Color32,
    pub cell_hover: Color32,
    pub flag_color: Color32,
    pub mine_color: Color32,
    pub accent_win: Color32,
    pub accent_lose: Color32,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            background: Color32::from_rgb(18, 18, 24),
            surface: Color32::from_rgb(30, 30, 40),
            surface_elevated: Color32::from_rgb(45, 45, 60),
            primary: Color32::from_rgb(72, 142, 242),
            primary_hover: Color32::from_rgb(92, 162, 252),
            secondary: Color32::from_rgb(60, 60, 80),
            text: Color32::from_rgb(240, 240, 250),
            text_secondary: Color32::from_rgb(160, 160, 180),
            cell_hidden: Color32::from_rgb(55, 55, 70),
            cell_revealed: Color32::from_rgb(40, 40, 55),
            cell_hover: Color32::from_rgb(70, 70, 90),
            flag_color: Color32::from_rgb(242, 72, 72),
            mine_color: Color32::from_rgb(50, 50, 60),
            accent_win: Color32::from_rgb(72, 200, 100),
            accent_lose: Color32::from_rgb(242, 72, 72),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color32::from_rgb(235, 235, 245),
            surface: Color32::from_rgb(250, 250, 255),
            surface_elevated: Color32::from_rgb(255, 255, 255),
            primary: Color32::from_rgb(48, 122, 232),
            primary_hover: Color32::from_rgb(68, 142, 252),
            secondary: Color32::from_rgb(200, 200, 210),
            text: Color32::from_rgb(30, 30, 40),
            text_secondary: Color32::from_rgb(100, 100, 120),
            cell_hidden: Color32::from_rgb(210, 210, 220),
            cell_revealed: Color32::from_rgb(225, 225, 235),
            cell_hover: Color32::from_rgb(190, 190, 205),
            flag_color: Color32::from_rgb(232, 72, 72),
            mine_color: Color32::from_rgb(80, 80, 90),
            accent_win: Color32::from_rgb(48, 180, 80),
            accent_lose: Color32::from_rgb(232, 72, 72),
        }
    }

    pub fn number_color(n: u8) -> Color32 {
        match n {
            1 => Color32::from_rgb(72, 142, 242),
            2 => Color32::from_rgb(72, 180, 100),
            3 => Color32::from_rgb(242, 72, 72),
            4 => Color32::from_rgb(142, 72, 242),
            5 => Color32::from_rgb(242, 142, 72),
            6 => Color32::from_rgb(72, 200, 200),
            7 => Color32::from_rgb(30, 30, 40),
            8 => Color32::from_rgb(128, 128, 140),
            _ => Color32::WHITE,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub colors: ThemeColors,
    pub is_dark: bool,
    pub corner_radius: f32,
    pub cell_spacing: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            colors: ThemeColors::dark(),
            is_dark: true,
            corner_radius: 8.0,
            cell_spacing: 2.0,
        }
    }

    pub fn light() -> Self {
        Self {
            colors: ThemeColors::light(),
            is_dark: false,
            corner_radius: 8.0,
            cell_spacing: 2.0,
        }
    }

    pub fn toggle(&mut self) {
        if self.is_dark {
            *self = Self::light();
        } else {
            *self = Self::dark();
        }
    }
}