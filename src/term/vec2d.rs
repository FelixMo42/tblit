use crate::term::Vec2;

pub struct Vec2d<T> {
    size: Vec2,
    data: Vec<T>
}

impl<T: Copy> Vec2d<T> {
    pub fn new(size: Vec2, init: T) -> Vec2d<T> {
        Vec2d {
            data: vec![init; (size.x * size.y) as usize],
            size,
        }
    }
}

impl<T> Vec2d<T> {
    pub fn get(&self, index: &Vec2) -> &T {
        &self.data[index.index(&self.size)]
    }

    pub fn get_mut(&mut self, index: &Vec2) -> &mut T {
        &mut self.data[index.index(&self.size)]
    }

    pub fn set(&mut self, index: &Vec2, value: T) {
        self.data[index.index(&self.size)] = value;
    }
}

impl Vec2 {
    fn index(&self, size: &Vec2) -> usize {
        return (self.y * size.x + self.x) as usize;
    }
}
