use crate::vector::Vector;
#[cfg(feature = "euclid")]
use euclid::Size2D;
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size<T> {
    pub width:  T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}

impl<T> From<Vector<T>> for Size<T> {
    fn from(vector: Vector<T>) -> Self {
        Size::new(vector.dx, vector.dy)
    }
}

impl<T: Mul> Size<T> {
    pub fn area(self) -> T::Output {
        self.width * self.height
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

impl<T: Copy + Mul<Output = T> + Div<Output = T> + Ord> Size<T> {
    pub fn scaled_to_fill(self, rhs: Size<T>) -> Size<T> {
        self * std::cmp::max(rhs.width / self.width, rhs.height / self.height)
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
            width:  self.width + rhs.width,
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
            width:  self.width * rhs,
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
            width:  self.width / rhs,
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
            width:  self.width % rhs,
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
