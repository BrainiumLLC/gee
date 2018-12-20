use crate::vector::Vector;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vector<RHS>> for Point<T> {
    type Output = Point<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vector<RHS>> for Point<T> {
    fn add_assign(&mut self, rhs: Vector<RHS>) {
        self.x += rhs.dx;
        self.y += rhs.dy
    }
}
