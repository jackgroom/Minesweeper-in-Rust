use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum CellType {
    Mine,
    Flag,
    Fog,
    Empty
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_type: CellType,
    is_opened: bool
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            cell_type: CellType::Fog,
            is_opened: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self.cell_type {
            CellType::Mine => '@',
            CellType::Flag => '?',
            CellType::Fog => '.',
            CellType::Empty => ' ',
        }
    }
}