use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CellType {
    Mine,
    Flag,
    Fog,
    Empty
}

impl CellType {
    pub fn to_char(&self) -> char {
        match self {
            CellType::Mine => '@',
            CellType::Flag => '?',
            CellType::Fog => '.',
            CellType::Empty => ' ',
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub is_marked: bool,
    pub is_opened: bool
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            cell_type: CellType::Empty,
            is_marked: false,
            is_opened: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}