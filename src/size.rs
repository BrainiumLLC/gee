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

impl<T: Mul<RHS, Output = Output>, RHS: Copy, Output> Mul<RHS> for Size<T> {
    type Output = Size<Output>;
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

impl<T: Div<RHS, Output = Output>, RHS: Copy, Output> Div<RHS> for Size<T> {
    type Output = Size<Output>;
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

impl<T: Rem<RHS, Output = Output>, RHS: Copy, Output> Rem<RHS> for Size<T> {
    type Output = Size<Output>;
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
