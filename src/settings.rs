use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom { width: usize, height: usize, mines: usize },
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Easy
    }
}

impl Difficulty {
    pub fn display_name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Custom { .. } => "Custom",
        }
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (30, 16, 99),
            Difficulty::Custom { width, height, mines } => (*width, *height, *mines),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub difficulty: Difficulty,
    pub dark_mode: bool,
    pub sound_enabled: bool,
    pub animations_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            dark_mode: true,
            sound_enabled: true,
            animations_enabled: true,
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        let (width, height, mines) = self.difficulty.dimensions();
        
        if width < 5 || width > 50 {
            return Err("Width must be between 5 and 50".to_string());
        }
        if height < 5 || height > 50 {
            return Err("Height must be between 5 and 50".to_string());
        }
        if mines < 1 {
            return Err("Must have at least 1 mine".to_string());
        }
        if mines >= width * height {
            return Err("Mines must be fewer than total cells".to_string());
        }
        
        Ok(())
    }
}