use crate::{scalar::Scalar, vector::Vector};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<T, Unit> {
    vector: Vector<T, Unit>,
}

impl<T, Unit> Point<T, Unit> {
    pub fn new(x: Scalar<T, Unit>, y: Scalar<T, Unit>) -> Self {
        Point {
            vector: Vector::new(x, y),
        }
    }

    pub fn x(&self) -> &Scalar<T, Unit> {
        &self.vector.dx
    }

    pub fn y(&self) -> &Scalar<T, Unit> {
        &self.vector.dy
    }
}
