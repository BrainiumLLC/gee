use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Vector<T> {
    pub fn new<U: Into<T>>(dx: U, dy: U) -> Self {
        Vector {
            dx: dx.into(),
            dy: dy.into(),
        }
    }

    pub fn as_ref(&self) -> Vector<&T> {
        Vector {
            dx: &self.dx,
            dy: &self.dy,
        }
    }

    pub fn as_mut(&mut self) -> Vector<&mut T> {
        Vector {
            dx: &mut self.dx,
            dy: &mut self.dy,
        }
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vector<RHS>> for Vector<T> {
    type Output = Vector<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Vector::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vector<RHS>> for Vector<T> {
    fn add_assign(&mut self, rhs: Vector<RHS>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy
    }
}

impl<T: Mul<RHS, Output = Output>, RHS: Copy, Output> Mul<RHS> for Vector<T> {
    type Output = Vector<Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Vector {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Vector<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.dx *= rhs;
        self.dy *= rhs
    }
}

impl<T: Div<RHS, Output = Output>, RHS: Copy, Output> Div<RHS> for Vector<T> {
    type Output = Vector<Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Vector {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Vector<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.dx /= rhs;
        self.dy /= rhs
    }
}

impl<T: Rem<RHS, Output = Output>, RHS: Copy, Output> Rem<RHS> for Vector<T> {
    type Output = Vector<Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Vector {
            dx: self.dx % rhs,
            dy: self.dy % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Vector<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.dx %= rhs;
        self.dy %= rhs
    }
}
