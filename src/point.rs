use crate::vector::Vector;
#[cfg(feature = "euclid")]
use euclid::Point2D;
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Add<RHS>, RHS> Add<Vector<RHS>> for Point<T> {
    type Output = Point<T::Output>;
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

impl<T: Sub<RHS>, RHS> Sub<Vector<RHS>> for Point<T> {
    type Output = Point<T::Output>;
    fn sub(self, rhs: Vector<RHS>) -> Self::Output {
        Point {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy,
        }
    }
}

impl<T: SubAssign<RHS>, RHS> SubAssign<Vector<RHS>> for Point<T> {
    fn sub_assign(&mut self, rhs: Vector<RHS>) {
        self.x -= rhs.dx;
        self.y -= rhs.dy
    }
}

impl<T: Mul<RHS>, RHS: Copy> Mul<RHS> for Point<T> {
    type Output = Point<T::Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Point<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.x *= rhs;
        self.y *= rhs
    }
}

impl<T: Zero> Point<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T> Point<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Point<U> {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Point2D<T>> for Point<T> {
    fn from(point: Point2D<T>) -> Self {
        Point::new(point.x, point.y)
    }
}

#[cfg(feature = "euclid")]
impl<T> Into<Point2D<T>> for Point<T> {
    fn into(self) -> Point2D<T> {
        Point2D::new(self.x, self.y)
    }
}
