use crate::Vector;
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

impl<T: en::Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    pub fn reposition_x(self, x: T) -> Self {
        Self::new(x, self.y)
    }

    pub fn reposition_y(self, y: T) -> Self {
        Self::new(self.x, y)
    }

    pub fn map<U: en::Num>(self, mut f: impl FnMut(T) -> U) -> Point<U> {
        Point::new(f(self.x), f(self.y))
    }

    impl_casts_and_cast!(Point);

    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn to_vector(self) -> Vector<T> {
        Vector::new(self.x, self.y)
    }

    pub fn move_to_by(self, to: Self, by: T) -> Self
    where
        T: en::Float,
    {
        self + (to - self).normalized() * by
    }
}

impl<T: en::Num> Add<Vector<T>> for Point<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl<T: en::Num> AddAssign<Vector<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub<Vector<T>> for Point<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x - rhs.dx, self.y - rhs.dy)
    }
}

impl<T: en::Num> Sub<Point<T>> for Point<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: en::Num> SubAssign<Vector<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs
    }
}

impl<T: en::Num> Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: en::Num> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: en::Num> Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: en::Num> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: en::Num> Rem<T> for Point<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: en::Num> RemAssign<T> for Point<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num, U> From<euclid::Point2D<T, U>> for Point<T> {
    fn from(point: euclid::Point2D<T, U>) -> Self {
        Point::new(point.x, point.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num, U> Into<euclid::Point2D<T, U>> for Point<T> {
    fn into(self) -> euclid::Point2D<T, U> {
        euclid::Point2D::new(self.x, self.y)
    }
}
