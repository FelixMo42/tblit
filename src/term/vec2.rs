use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}
impl Vec2 {
    pub fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {    
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {    
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Into<Vec2> for (isize, isize) {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.0,
            y: self.1,
        }
    }
}
