use crate::vec2::Vec2;
#[cfg(feature = "euclid")]
use euclid::Size2D;
use num_traits::{real::Real, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }

    pub fn into_vec2(self) -> Vec2<T> {
        Vec2::from(self)
    }
}

impl<T: Copy> Size<T> {
    pub fn square(dim: T) -> Self {
        Size {
            width: dim,
            height: dim,
        }
    }
}

impl<T> From<Vec2<T>> for Size<T> {
    fn from(vector: Vec2<T>) -> Self {
        Size::new(vector.dx, vector.dy)
    }
}

impl<T: Mul> Size<T> {
    pub fn area(self) -> T::Output {
        self.width * self.height
    }
}

impl<T: Div> Size<T> {
    pub fn aspect_ratio(self) -> T::Output {
        self.width / self.height
    }
}

impl<T: Ord> Size<T> {
    pub fn min(self) -> T {
        std::cmp::min(self.width, self.height)
    }
    pub fn max(self) -> T {
        std::cmp::max(self.width, self.height)
    }
}

impl<T: Mul<Output = T>> Size<T> {
    pub fn scaled(self, rhs: Vec2<T>) -> Self {
        Self {
            width: self.width * rhs.dx,
            height: self.height * rhs.dy,
        }
    }
}

impl<T: Copy + Mul<Output = T> + Div<Output = T>> Size<T> {
    pub fn scaled_to_width(self, rhs: T) -> Size<T> {
        Size::new(rhs, self.height * rhs / self.width)
    }

    pub fn scaled_to_height(self, rhs: T) -> Size<T> {
        Size::new(self.width * rhs / self.height, rhs)
    }
}

impl<T: Copy + Mul<Output = T> + Div<Output = T> + PartialOrd> Size<T> {
    pub fn scaled_to_fill(self, rhs: Size<T>) -> Size<T> {
        let width = rhs.width / self.width;
        let height = rhs.height / self.height;
        self * if width > height { width } else { height }
    }
}

impl<T: Div<Output = T> + Mul<Output = T> + Real> Size<T> {
    /// Downscales the size to fit within `rhs` while preserving aspect ratio.
    pub fn scaled_to_fit(&self, rhs: Size<T>) -> Size<T> {
        self.scaled_to_fill_and_fit(Self::new(
            self.width.min(rhs.width),
            self.height.min(rhs.height),
        ))
    }

    /// Scales the size to fit within `rhs` while preserving aspect ratio.
    /// The largest size that fits within `rhs` will be calculated.
    pub fn scaled_to_fill_and_fit(&self, rhs: Size<T>) -> Size<T> {
        let aspect_ratio = self.aspect_ratio();
        let width = rhs.width.min(rhs.height * aspect_ratio);
        let height = rhs.height.min(rhs.width * aspect_ratio.recip());
        Self::new(width, height)
    }
}

impl<T: Ord> Size<T> {
    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    pub fn is_portrait(&self) -> bool {
        self.width < self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Size<RHS>> for Size<T> {
    type Output = Size<Output>;
    fn add(self, rhs: Size<RHS>) -> Self::Output {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Size<RHS>> for Size<T> {
    fn add_assign(&mut self, rhs: Size<RHS>) {
        self.width += rhs.width
    }
}

impl<T: Mul<RHS>, RHS: Copy> Mul<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Size {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Size<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.width *= rhs;
        self.height *= rhs
    }
}

impl<T: Div<RHS>, RHS: Copy> Div<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Size {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Size<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.width /= rhs;
        self.height /= rhs
    }
}

impl<T: Rem<RHS>, RHS: Copy> Rem<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Size {
            width: self.width % rhs,
            height: self.height % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Size<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.width %= rhs;
        self.height %= rhs
    }
}

impl<T: Zero> Size<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T> Size<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Size<U> {
        Size {
            width: f(self.width),
            height: f(self.height),
        }
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Size2D<T>> for Size<T> {
    fn from(size: Size2D<T>) -> Self {
        Size::new(size.width, size.height)
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<Size2D<T>> for Size<T> {
    fn into(self) -> Size2D<T> {
        Size2D::new(self.width, self.height)
    }
}
