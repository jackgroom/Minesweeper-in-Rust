use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(self: &Vec2, other_vec2: &Vec2) -> Vec2 {
        Self::new(self.x + other_vec2.x, self.y + other_vec2.y)
    }

    pub fn sub(self: &Vec2, other_vec2: &Vec2) -> Vec2 {
        Self::new(self.x - other_vec2.x, self.y - other_vec2.y)
    }

    pub fn mult(&mut self, multiplier: i32) {
        self.x = self.x * multiplier;
        self.y = self.y * multiplier;
    }

    pub fn div(&mut self, division: i32) {
        self.x /= division;
        self.y /= division;
    }
}
