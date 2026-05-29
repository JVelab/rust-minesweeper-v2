use crate::board::Board;
use crate::settings::Difficulty;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    NotStarted,
    Running,
    Won,
    Lost,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub time_elapsed: Duration,
    pub start_time: Option<Instant>,
}

impl Game {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        Self {
            board: Board::new(width, height, mine_count),
            state: GameState::NotStarted,
            time_elapsed: Duration::ZERO,
            start_time: None,
        }
    }

    pub fn from_difficulty(difficulty: Difficulty) -> Self {
        let (width, height, mines) = match difficulty {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (30, 16, 99),
            Difficulty::Custom { width, height, mines } => (width, height, mines),
        };
        Self::new(width, height, mines)
    }

    pub fn reset(&mut self, width: usize, height: usize, mine_count: usize) {
        *self = Self::new(width, height, mine_count);
    }

    pub fn reset_with_difficulty(&mut self, difficulty: Difficulty) {
        *self = Self::from_difficulty(difficulty);
    }

    pub fn start(&mut self) {
        if self.state == GameState::NotStarted {
            self.state = GameState::Running;
            self.start_time = Some(Instant::now());
        }
    }

    pub fn update_time(&mut self) {
        if self.state == GameState::Running {
            if let Some(start) = self.start_time {
                self.time_elapsed = start.elapsed();
            }
        }
    }

    pub fn reveal(&mut self, row: usize, col: usize) -> bool {
        if self.state == GameState::Won || self.state == GameState::Lost {
            return false;
        }
        
        if self.state == GameState::NotStarted {
            let exploded = self.board.safe_reveal_first(row, col);
            self.state = GameState::Running;
            self.start_time = Some(Instant::now());
            return exploded;
        }
        
        self.board.reveal_cell(row, col)
    }

    pub fn flag(&mut self, row: usize, col: usize) {
        if self.state == GameState::Running {
            self.board.toggle_flag(row, col);
        }
    }

    pub fn question(&mut self, row: usize, col: usize) {
        if self.state == GameState::Running {
            self.board.toggle_question(row, col);
        }
    }

    pub fn check_game_end(&mut self) {
        if self.state != GameState::Running {
            return;
        }
        
        if self.board.check_victory() {
            self.state = GameState::Won;
        }
    }

    pub fn get_mines_remaining(&self) -> isize {
        self.board.mine_count as isize - self.board.flag_count as isize
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::Won || self.state == GameState::Lost
    }

    pub fn format_time(&self) -> String {
        let total_seconds = self.time_elapsed.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:03}:{:02}", minutes, seconds)
    }
}