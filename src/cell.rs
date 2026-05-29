#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
    Questioned,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub is_mine: bool,
    pub state: CellState,
    pub adjacent_mines: u8,
    pub is_exploded: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            is_mine: false,
            state: CellState::Hidden,
            adjacent_mines: 0,
            is_exploded: false,
        }
    }
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mine(mut self) -> Self {
        self.is_mine = true;
        self
    }

    pub fn reveal(&mut self) {
        if self.state != CellState::Flagged {
            self.state = CellState::Revealed;
        }
    }

    pub fn toggle_flag(&mut self) {
        match self.state {
            CellState::Hidden => self.state = CellState::Flagged,
            CellState::Flagged => self.state = CellState::Hidden,
            _ => {}
        }
    }

    pub fn toggle_question(&mut self) {
        match self.state {
            CellState::Hidden => self.state = CellState::Questioned,
            CellState::Questioned => self.state = CellState::Hidden,
            _ => {}
        }
    }

    pub fn is_revealed(&self) -> bool {
        self.state == CellState::Revealed
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.state, CellState::Hidden | CellState::Flagged | CellState::Questioned)
    }

    pub fn is_flagged(&self) -> bool {
        self.state == CellState::Flagged
    }
}