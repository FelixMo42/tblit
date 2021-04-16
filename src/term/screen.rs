use super::*;
use std::sync::{Arc, Mutex};

pub struct Screen {
    pub fg: Vec2d<Color>,
    pub bg: Vec2d<Color>,
    pub chars: Vec2d<char>,
    pub size: Vec2,
}

const DEF_FG: Color = Color(200, 200, 200);
const DEF_BG: Color = Color(  0,   0,   0);

impl Screen {
    pub fn new(size: Vec2) -> Arc<Mutex<Screen>> {
        return Arc::new(Mutex::new(Screen {
            fg: Vec2d::new(size, DEF_FG),
            bg: Vec2d::new(size, DEF_BG),
            chars: Vec2d::new(size, ' '),
            size,
        }));
    }

    pub fn set(&mut self, index: &Vec2, chr: char) {
        self.chars.set(index, chr);
    }

    pub fn set_fg(&mut self, index: &Vec2, color: Color) {
        self.fg.set(index, color);
    }

    pub fn set_bg(&mut self, index: &Vec2, color: Color) {
        self.bg.set(index, color);
    }
}
