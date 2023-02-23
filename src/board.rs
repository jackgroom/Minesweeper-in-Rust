use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use crate::cell::Cell;
use crate::vec2::Vec2;

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
        for y in 0 .. self.cols {
            for x in 0 .. self.rows {
                let current_pos: Vec2 = Vec2::new(x as i32, y as i32);
                let current_cell: Cell = self[current_pos];


                if self.player_pos == current_pos {
                    print!("[{}]", current_cell)
                } else {
                    print!(" {} ", current_cell)
                }
            }
            println!();
        }
    }

    pub fn update_player_pos(&mut self, vec: Vec2) {
        self.player_pos = self.player_pos.add(&vec);
    }
}