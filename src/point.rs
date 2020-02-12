use crate::{OrdinaryNum, Vec2};
#[cfg(feature = "euclid")]
use euclid::Point2D;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: OrdinaryNum> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    pub fn map<U: OrdinaryNum>(self, mut f: impl FnMut(T) -> U) -> Point<U> {
        Point::new(f(self.x), f(self.x))
    }

    impl_casts_and_cast!(Point);

    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn to_vec2(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }
}

impl<T: OrdinaryNum> Add<Vec2<T>> for Point<T> {
    type Output = Self;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Point::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl<T: OrdinaryNum> AddAssign<Vec2<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Sub<Vec2<T>> for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Point::new(self.x - rhs.dx, self.y - rhs.dy)
    }
}

impl<T: OrdinaryNum> Sub<Point<T>> for Point<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: OrdinaryNum> SubAssign<Vec2<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = *self - rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Point<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Point<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<Point2D<T>> for Point<T> {
    fn from(point: Point2D<T>) -> Self {
        Point::new(point.x, point.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<Point2D<T>> for Point<T> {
    fn into(self) -> Point2D<T> {
        Point2D::new(self.x, self.y)
    }
}
