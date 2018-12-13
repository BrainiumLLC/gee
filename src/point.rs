use crate::vector::Vector;
use std::ops::Add;

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

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vector<RHS>> for Point<T> {
    type Output = Point<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Point {
            vector: self.vector + rhs,
        }
    }
}

impl<'a, T: Add<&'a RHS, Output = Output>, RHS, Output> Add<&'a Vector<RHS>> for Point<T> {
    type Output = Point<Output>;
    fn add(self, rhs: &'a Vector<RHS>) -> Self::Output {
        Point {
            vector: self.vector + rhs,
        }
    }
}

impl<'a, T, RHS, Output> Add<Vector<RHS>> for &'a Point<T>
where
    &'a T: Add<RHS, Output = Output>,
{
    type Output = Point<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Point {
            vector: &self.vector + rhs,
        }
    }
}

impl<'a, 'b, T, RHS, Output> Add<&'b Vector<RHS>> for &'a Point<T>
where
    &'a T: Add<&'b RHS, Output = Output>,
{
    type Output = Point<Output>;
    fn add(self, rhs: &'b Vector<RHS>) -> Self::Output {
        Point {
            vector: &self.vector + rhs,
        }
    }
}
