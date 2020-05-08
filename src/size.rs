use crate::Vector;
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

impl<T: en::Num> Size<T> {
    pub fn new_unchecked(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn new(width: T, height: T) -> Self {
        if cfg!(not(feature = "unchecked-ctors")) {
            Self::try_new(width, height).expect("width or height is less than 0")
        } else {
            Self::new_unchecked(width, height)
        }
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

    pub fn resize_width(self, width: T) -> Self {
        Self::new(width, self.height)
    }

    pub fn resize_height(self, height: T) -> Self {
        Self::new(self.width, height)
    }

    pub fn scale(self, scale: Vector<T>) -> Self {
        self.scale_width(scale.dx).scale_height(scale.dy)
    }

    pub fn scale_width(self, coeff: T) -> Self {
        self.resize_width(self.width * coeff)
    }

    pub fn scale_height(self, coeff: T) -> Self {
        self.resize_height(self.height * coeff)
    }

    pub fn scale_uniform(self, coeff: T) -> Self {
        self * coeff
    }

    pub fn fit_width(self, width_to_fit: T) -> Size<T> {
        Self::new(width_to_fit, self.height * width_to_fit / self.width)
    }

    pub fn fit_height(self, height_to_fit: T) -> Size<T> {
        Self::new(self.width * height_to_fit / self.height, height_to_fit)
    }

    pub fn fill(self, rhs: Size<T>) -> Size<T> {
        let width = rhs.width / self.width;
        let height = rhs.height / self.height;
        self.scale_uniform(width.max(height))
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

    pub fn map<U: en::Num>(self, mut f: impl FnMut(T) -> U) -> Size<U> {
        Size::new(f(self.width), f(self.height))
    }

    impl_casts_and_cast!(Size);

    pub fn to_array(self) -> [T; 2] {
        [self.width, self.height]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }

    pub fn to_vector(self) -> Vector<T> {
        Vector::from(self)
    }
}

impl<T: en::Num> Add for Size<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T: en::Num> AddAssign for Size<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Mul<T> for Size<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: en::Num> MulAssign<T> for Size<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: en::Num> Div<T> for Size<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: en::Num> DivAssign<T> for Size<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: en::Num> Rem<T> for Size<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: en::Num> RemAssign<T> for Size<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<T: en::Num> From<Vector<T>> for Size<T> {
    fn from(vector: Vector<T>) -> Self {
        Self::new(vector.dx, vector.dy)
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num> From<Size2D<T>> for Size<T> {
    fn from(size: Size2D<T>) -> Self {
        Self::new(size.width, size.height)
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num> Into<Size2D<T>> for Size<T> {
    fn into(self) -> Size2D<T> {
        Size2D::new(self.width, self.height)
    }
}
