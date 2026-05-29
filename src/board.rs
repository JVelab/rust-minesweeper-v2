use crate::cell::{Cell, CellState};
use rand::seq::IteratorRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub mine_count: usize,
    pub flag_count: usize,
    pub revealed_count: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let mut board = Self {
            width,
            height,
            cells: vec![vec![Cell::new(); width]; height],
            mine_count,
            flag_count: 0,
            revealed_count: 0,
        };
        board.place_mines(mine_count);
        board.calculate_adjacent();
        board
    }

    fn place_mines(&mut self, count: usize) {
        let mut rng = thread_rng();
        let total_cells = self.width * self.height;
        let mine_positions: Vec<usize> = (0..total_cells)
            .choose_multiple(&mut rng, count);
        
        for pos in mine_positions {
            let row = pos / self.width;
            let col = pos % self.width;
            self.cells[row][col].is_mine = true;
        }
    }

    fn calculate_adjacent(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if !self.cells[row][col].is_mine {
                    self.cells[row][col].adjacent_mines = self.count_adjacent_mines(row, col);
                }
            }
        }
    }

    fn count_adjacent_mines(&self, row: usize, col: usize) -> u8 {
        let mut count = 0u8;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if self.is_valid_pos(nr, nc) {
                    if self.cells[nr as usize][nc as usize].is_mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn is_valid_pos(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.height as isize && col >= 0 && col < self.width as isize
    }

    pub fn reveal_cell(&mut self, row: usize, col: usize) -> bool {
        if !self.is_valid_pos(row as isize, col as isize) {
            return false;
        }
        
        let cell = &mut self.cells[row][col];
        if !cell.is_hidden() {
            return false;
        }
        
        if cell.is_flagged() {
            self.flag_count -= 1;
        }
        
        cell.reveal();
        self.revealed_count += 1;
        
        if cell.is_mine {
            cell.is_exploded = true;
            return true;
        }
        
        if cell.adjacent_mines == 0 {
            self.flood_fill(row as isize, col as isize);
        }
        
        false
    }

    fn flood_fill(&mut self, row: isize, col: isize) {
        let mut stack = vec![(row, col)];
        
        while let Some((r, c)) = stack.pop() {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 { continue; }
                    let nr = r + dr;
                    let nc = c + dc;
                    
                    if !self.is_valid_pos(nr, nc) { continue; }
                    
                    let cell = &mut self.cells[nr as usize][nc as usize];
                    if !cell.is_hidden() { continue; }
                    if cell.is_flagged() { continue; }
                    
                    cell.reveal();
                    self.revealed_count += 1;
                    
                    if cell.adjacent_mines == 0 && !cell.is_mine {
                        stack.push((nr, nc));
                    }
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) {
        if !self.is_valid_pos(row as isize, col as isize) {
            return;
        }
        
        let cell = &mut self.cells[row][col];
        if !cell.is_hidden() {
            return;
        }
        
        if cell.is_flagged() {
            self.flag_count -= 1;
        } else {
            self.flag_count += 1;
        }
        
        cell.toggle_flag();
    }

    pub fn toggle_question(&mut self, row: usize, col: usize) {
        if !self.is_valid_pos(row as isize, col as isize) {
            return;
        }
        
        let cell = &mut self.cells[row][col];
        if !matches!(cell.state, CellState::Hidden) {
            return;
        }
        
        cell.toggle_question();
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        if row >= self.height || col >= self.width {
            None
        } else {
            Some(&self.cells[row][col])
        }
    }

    pub fn safe_reveal_first(&mut self, row: usize, col: usize) -> bool {
        if self.cells[row][col].is_mine {
            let mut rng = thread_rng();
            let mut candidates: Vec<(usize, usize)> = Vec::new();
            
            for r in 0..self.height {
                for c in 0..self.width {
                    if !self.cells[r][c].is_mine && !(r == row && c == col) {
                        candidates.push((r, c));
                    }
                }
            }
            
            if let Some((new_r, new_c)) = candidates.iter().choose(&mut rng) {
                self.cells[row][col].is_mine = false;
                self.cells[*new_r][*new_c].is_mine = true;
                self.calculate_adjacent();
            }
        }
        
        self.reveal_cell(row, col)
    }

    pub fn total_cells(&self) -> usize {
        self.width * self.height
    }

    pub fn non_mine_cells(&self) -> usize {
        self.total_cells() - self.mine_count
    }

    pub fn check_victory(&self) -> bool {
        self.revealed_count == self.non_mine_cells()
    }

    pub fn reveal_all_mines(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[row][col].is_mine && !self.cells[row][col].is_flagged() {
                    self.cells[row][col].reveal();
                }
            }
        }
    }
}