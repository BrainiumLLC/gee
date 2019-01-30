use crate::size::Size;
#[cfg(feature = "euclid")]
use euclid::Vector2D;
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    pub fn dot_product<RHS, A>(self, rhs: Vector<RHS>) -> A::Output
    where
        T: Mul<RHS, Output = A>,
        A: Add<A>,
    {
        self.dx * rhs.dx + self.dy * rhs.dy
    }

    pub fn magnitude_squared<A>(self) -> A::Output
    where
        T: Mul<Output = A> + Copy,
        A: Add<A>,
    {
        self.dot_product(self)
    }

    pub fn magnitude<A>(self) -> A::Output
    where
        T: Mul<Output = A> + Copy,
        A::Output: Float,
        A: Add,
    {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized<A>(self) -> Vector<<T as Div<A::Output>>::Output>
    where
        T: Mul<Output = A> + Copy + Div<A::Output>,
        A::Output: Float,
        A: Add,
    {
        self / self.magnitude()
    }
}

impl<T> From<Size<T>> for Vector<T> {
    fn from(size: Size<T>) -> Self {
        Vector::new(size.width, size.height)
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

impl<T: Neg<Output = T>> Vector<T> {
    pub fn perpendicular(self) -> Self {
        Self::new(-self.dy, self.dx)
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Vector2D<T>> for Vector<T> {
    fn from(vector: Vector2D<T>) -> Self {
        Vector::new(vector.x, vector.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<Vector2D<T>> for Vector<T> {
    fn into(self) -> Vector2D<T> {
        Vector2D::new(self.dx, self.dy)
    }
}
