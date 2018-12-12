use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<T> {
    vector: Vector<T>,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            vector: Vector::new(x, y),
        }
    }

    pub fn x(&self) -> &T {
        &self.vector.dx
    }

    pub fn y(&self) -> &T {
        &self.vector.dy
    }
}
