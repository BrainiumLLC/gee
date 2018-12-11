use crate::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Sub};

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

impl<T: Sub<Output = Output>, Unit, Output> Sub<Point<T, Unit>> for Point<T, Unit> {
    type Output = Vector<Output, Unit>;
    fn sub(self, p: Point<T, Unit>) -> Self::Output {
        self.vector - p.vector
    }
}

impl<T: Add<Output = Output>, Unit, Output> Add<Vector<T, Unit>> for Point<T, Unit> {
    type Output = Point<Output, Unit>;
    fn add(self, v: Vector<T, Unit>) -> Self::Output {
        Point { vector: self.vector + v }
    }
}

impl<T: Sub<Output = Output>, Unit, Output> Sub<Vector<T, Unit>> for Point<T, Unit> {
    type Output = Point<Output, Unit>;
    fn sub(self, v: Vector<T, Unit>) -> Self::Output {
        Point { vector: self.vector - v }
    }
}
