use std::{cmp::Ordering, fmt::Display, iter::Step};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<T: Copy> {
    pub x: T,
    pub y: T
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        return Vec2 { x, y };
    }

    pub fn to(self, target: Vec2<T>) -> Rect<T> {
        return Rect(self, target);
    }
}

impl Vec2<usize> {
    pub fn index(&self, index: &Vec2<usize>) -> usize {
        return index.x + self.x * index.y;
    }
}

impl<T: Ord + Copy> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, outher: &Vec2<T>) -> Option<Ordering> {
        return Some(self.cmp(outher));
    }
}

impl<T: Ord + Copy> Ord for Vec2<T> {
    fn cmp(&self, outher: &Vec2<T>) -> Ordering {
        if self.x > outher.x && self.y > outher.y {
            return Ordering::Greater;
        } else if self == outher {
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    }
}


impl<T: Display + Copy> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.x, self.y).as_str())
    }
}

#[derive(Clone, Copy)]
pub struct Rect<T: Copy> (pub Vec2<T>, pub Vec2<T>);

impl<T: Step + Copy> IntoIterator for Rect<T> {
    type Item = Vec2<T>;
    type IntoIter = RectIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        return RectIter {
            cord: self.0.clone(),
            rect: self,
        };
    }
}

impl<T: Step + Copy + Ord> Rect<T> {
    pub fn fit(&self, cord: &Vec2<T>) -> Rect<T> {
        let mut new = self.clone();

        if cord.x > self.1.x {
            new.1.x = cord.x
        }
        if cord.x < self.0.x {
            new.0.x = cord.x
        }
        if cord.y > self.1.y {
            new.1.y = cord.y
        }
        if cord.y < self.0.y {
            new.0.y = cord.y
        }

        return new
    }
}

impl<T: Display + Copy> Display for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.0, self.1).as_str())
    }
}

pub struct RectIter<T: Step + Copy> {
    rect: Rect<T>,
    cord: Vec2<T>
}

impl<T: Step + Copy> Iterator for RectIter<T> {
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Vec2<T>> {
        // We've reached the end of the rect, lets exit.
        if self.cord.y == self.rect.1.y {
            return None;
        }

        // Clone the cord now, so that we can return it after updating it.
        let cord = self.cord.clone();

        // Move forwards one along the x axis.
        self.cord.x = Step::forward(self.cord.x, 1);

        // We've reached the end of the x axis, go to start of next line.
        if self.cord.x == self.rect.1.x {
            self.cord.x = self.rect.0.x;
            self.cord.y = Step::forward(self.cord.y, 1);
        }

        // Return the pre-update cord.
        return Some(cord);
    }
}
