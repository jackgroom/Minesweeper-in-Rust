use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::io::{Write, stdout};
use crossterm::ExecutableCommand;

use crate::cell::{Cell, CellType};
use crate::vec2::Vec2;

use crossterm::terminal::{Clear, ClearType};

pub struct Board {
    rows: usize,
    cols: usize,
    elems: Vec<Cell>,
    player_pos: Vec2,
}

impl Index<Vec2> for Board {
    type Output = Cell;
    fn index(&self, index: Vec2) -> &Self::Output {
        let _x: usize = index.x as usize;
        let _y: usize = index.y as usize;
        &self.elems[_x + self.rows - 1 + _y]
    }
}

impl IndexMut<Vec2> for Board {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        let _x: usize = index.x as usize;
        let _y: usize = index.y as usize;
        &mut self.elems[_x + self.rows - 1 + _y]
    }
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            elems: vec![Cell::default(); rows * cols],
            player_pos: Vec2::new(0, 0),
        }
    }

    pub fn get_elems(&self) -> &Vec<Cell> {
        &self.elems
    }

    pub fn display(&self) {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All)).expect("Error clearing screen buffer");

        for y in 0 .. self.cols {
            for x in 0 .. self.rows {
                let current_pos: Vec2 = Vec2::new(x as i32, y as i32);
                let current_cell: Cell = self[current_pos];

                let type_to_print: CellType =
                    if current_cell.is_opened {
                        current_cell.cell_type
                    } else {
                        if current_cell.is_marked { CellType::Flag }
                        else { CellType::Fog }
                    };

                if self.player_pos == current_pos {
                    print!("[{}]", type_to_print);
                } else {
                    print!(" {} ", type_to_print);
                }
            }
            println!();
        }
    }

    pub fn update_player_pos(&mut self, vec: Vec2) {
        self.player_pos = self.player_pos.add(&vec);

        // checking bounds
        let casted_rows: i32 = self.rows as i32;
        let casted_cols: i32 = self.cols as i32;

        // probably a simpler approach to this at the cost of readability
        if self.player_pos.x > casted_rows - 1 {
            self.player_pos.x = casted_rows - 1;
        } else if self.player_pos.x < 0 {
            self.player_pos.x = 0;
        } else if self.player_pos.y > casted_cols - 1 {
            self.player_pos.y = casted_cols - 1
        } else if self.player_pos.y < 0 {
            self.player_pos.y = 0;
        }
    }

    pub fn toggle_cell_flag(&mut self) {
        let current_position: Vec2 = self.player_pos;
        let mut current_cell: Cell = self[current_position];


        if !current_cell.is_opened {
           self[current_position].is_marked = !self[current_position].is_marked;
        }
    }

    pub fn show_cell(&mut self) {
        let current_position: Vec2 = self.player_pos;
        self[current_position].is_opened = !self[current_position].is_opened;
    }
}