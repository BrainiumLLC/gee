use crate::vec2::Vec2;
#[cfg(feature = "euclid")]
use euclid::Point2D;
use num_traits::{Zero, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn move_to_by(self, to: Self, by: T) -> Self where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Float {
        self + (to - self).normalized() * by
    }

    pub fn into_vec2(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }
}

impl<T: Add<RHS>, RHS> Add<Vec2<RHS>> for Point<T> {
    type Output = Point<T::Output>;
    fn add(self, rhs: Vec2<RHS>) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vec2<RHS>> for Point<T> {
    fn add_assign(&mut self, rhs: Vec2<RHS>) {
        self.x += rhs.dx;
        self.y += rhs.dy
    }
}

impl<T: Sub<RHS>, RHS> Sub<Vec2<RHS>> for Point<T> {
    type Output = Point<T::Output>;
    fn sub(self, rhs: Vec2<RHS>) -> Self::Output {
        Point {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy,
        }
    }
}

impl<T: Sub<RHS>, RHS> Sub<Point<RHS>> for Point<T> {
    type Output = Vec2<T::Output>;
    fn sub(self, rhs: Point<RHS>) -> Self::Output {
        Vec2 {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign<RHS>, RHS> SubAssign<Vec2<RHS>> for Point<T> {
    fn sub_assign(&mut self, rhs: Vec2<RHS>) {
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
