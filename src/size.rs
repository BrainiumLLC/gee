use crate::{OrdinaryNum, Vec2};
#[cfg(feature = "euclid")]
use euclid::Size2D;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: check size validity in deserialize
pub struct Size<T> {
    width:  T,
    height: T,
}

impl<T: OrdinaryNum> Size<T> {
    pub fn new_unchecked(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn new(width: T, height: T) -> Self {
        Self::try_new(width, height).expect("width or height is less than 0")
    }

    pub fn try_new(width: T, height: T) -> Option<Self> {
        if width >= T::zero() && height >= T::zero() {
            Some(Self::new_unchecked(width, height))
        } else {
            None
        }
    }

    pub fn square(dim: T) -> Self {
        Self::new(dim, dim)
    }

    pub fn zero() -> Self {
        Self::square(T::zero())
    }

    pub fn width(self) -> T {
        self.width
    }

    pub fn height(self) -> T {
        self.height
    }

    pub fn is_landscape(self) -> bool {
        self.width > self.height
    }

    pub fn is_portrait(self) -> bool {
        self.width < self.height
    }

    pub fn is_square(self) -> bool {
        self.width == self.height
    }

    pub fn area(self) -> T {
        self.width * self.height
    }

    pub fn aspect_ratio(self) -> T {
        self.width / self.height
    }

    pub fn min_dim(self) -> T {
        self.width.min(self.height)
    }

    pub fn max_dim(self) -> T {
        self.width.max(self.height)
    }

    pub fn scaled(self, rhs: T) -> Self {
        Self::new(self.width * rhs, self.height * rhs)
    }

    pub fn scaled_by_vec2(self, rhs: Vec2<T>) -> Self {
        Self::new(self.width * rhs.dx, self.height * rhs.dy)
    }

    pub fn fit_width(self, rhs: T) -> Size<T> {
        Self::new(rhs, self.height * rhs / self.width)
    }

    pub fn fit_height(self, rhs: T) -> Size<T> {
        Self::new(self.width * rhs / self.height, rhs)
    }

    pub fn fill(self, rhs: Size<T>) -> Size<T> {
        let width = rhs.width / self.width;
        let height = rhs.height / self.height;
        self.scaled(width.max(height))
    }

    /// Downscales the size to fit within `rhs` while preserving aspect ratio.
    pub fn fit(self, rhs: Size<T>) -> Size<T> {
        self.fill_and_fit(Self::new(
            self.width.min(rhs.width),
            self.height.min(rhs.height),
        ))
    }

    /// Scales the size to fit within `rhs` while preserving aspect ratio.
    /// The largest size that fits within `rhs` will be calculated.
    pub fn fill_and_fit(self, rhs: Size<T>) -> Size<T> {
        let aspect_ratio = self.aspect_ratio();
        let width = rhs.width.min(rhs.height * aspect_ratio);
        let height = rhs.height.min(rhs.width / aspect_ratio);
        Self::new(width, height)
    }

    pub fn map<U: OrdinaryNum, F: Fn(T) -> U>(self, f: F) -> Size<U> {
        Size::new(f(self.width), f(self.height))
    }

    pub fn into_vec2(self) -> Vec2<T> {
        Vec2::from(self)
    }
}

impl<T: OrdinaryNum> Add for Size<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T: OrdinaryNum> AddAssign for Size<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Size<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Size<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Size<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Size<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Size<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Size<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<T: OrdinaryNum> From<Vec2<T>> for Size<T> {
    fn from(vector: Vec2<T>) -> Self {
        Self::new(vector.dx, vector.dy)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<Size2D<T>> for Size<T> {
    fn from(size: Size2D<T>) -> Self {
        Self::new(size.width, size.height)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<Size2D<T>> for Size<T> {
    fn into(self) -> Size2D<T> {
        Size2D::new(self.width, self.height)
    }
}
