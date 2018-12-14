use crate::vector::Vector;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size<T> {
    vector: Vector<T>,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size {
            vector: Vector::new(width, height),
        }
    }

    pub fn width(&self) -> &T {
        &self.vector.dx
    }

    pub fn height(&self) -> &T {
        &self.vector.dy
    }
}

impl<T: Mul> Size<T> {
    pub fn area(self) -> T::Output {
        self.vector.dx * self.vector.dy
    }
}

impl<T: Copy + Mul<Output = T> + Div<Output = T>> Size<T> {
    pub fn scaled_to_width(self, rhs: T) -> Size<T> {
        Size::new(rhs, *self.height() * rhs / *self.width())
    }

    pub fn scaled_to_height(self, rhs: T) -> Size<T> {
        Size::new(*self.width() * rhs / *self.height(), rhs)
    }
}

impl<T: Copy + Mul<Output = T> + Div<Output = T> + Ord> Size<T> {
    pub fn scaled_to_fill(self, rhs: Size<T>) -> Size<T> {
        Size {
            vector: self.vector
                * std::cmp::max(*rhs.width() / *self.width(), *rhs.height() / *self.height()),
        }
    }
}

impl<T: Ord> Size<T> {
    pub fn is_landscape(&self) -> bool {
        self.width() > self.height()
    }

    pub fn is_portrait(&self) -> bool {
        self.width() < self.height()
    }

    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Size<RHS>> for Size<T> {
    type Output = Size<Output>;
    fn add(self, rhs: Size<RHS>) -> Self::Output {
        Size {
            vector: self.vector + rhs.vector,
        }
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Size<RHS>> for Size<T> {
    fn add_assign(&mut self, rhs: Size<RHS>) {
        self.vector += rhs.vector
    }
}

impl<T: Mul<RHS>, RHS: Copy> Mul<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Size {
            vector: self.vector * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Size<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.vector *= rhs
    }
}

impl<T: Div<RHS>, RHS: Copy> Div<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Size {
            vector: self.vector / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Size<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.vector /= rhs
    }
}

impl<T: Rem<RHS>, RHS: Copy> Rem<RHS> for Size<T> {
    type Output = Size<T::Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Size {
            vector: self.vector % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Size<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.vector %= rhs
    }
}
