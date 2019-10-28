use crate::{vec2::Vec2, vec4::Vec4};
#[cfg(feature = "euclid")]
use euclid::Vector3D;
use num_traits::{Float, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec3<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
}

impl<T> Vec3<T> {
    pub const fn new(dx: T, dy: T, dz: T) -> Self {
        Vec3 { dx, dy, dz }
    }

    pub fn from_vec2(vec2: Vec2<T>) -> Self
    where
        T: Zero,
    {
        Self::new(vec2.dx, vec2.dy, Zero::zero())
    }

    pub fn from_vec4(vec4: Vec4<T>) -> Self {
        Self::new(vec4.dx, vec4.dy, vec4.dz)
    }

    pub const fn as_ref(&self) -> Vec3<&T> {
        Vec3 {
            dx: &self.dx,
            dy: &self.dy,
            dz: &self.dz,
        }
    }

    pub fn as_mut(&mut self) -> Vec3<&mut T> {
        Vec3 {
            dx: &mut self.dx,
            dy: &mut self.dy,
            dz: &mut self.dz,
        }
    }

    pub fn dot_product<RHS, A>(self, rhs: Vec3<RHS>) -> A::Output
    where
        T: Mul<RHS, Output = A>,
        A: Add<Output = A>,
    {
        self.dx * rhs.dx + self.dy * rhs.dy + self.dz * rhs.dz
    }

    pub fn magnitude_squared<A>(self) -> A::Output
    where
        T: Mul<Output = A> + Copy,
        A: Add<Output = A>,
    {
        self.dot_product(self)
    }

    pub fn magnitude<A>(self) -> A::Output
    where
        T: Mul<Output = A> + Copy,
        A::Output: Float,
        A: Add<Output = A>,
    {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized<A>(self) -> Vec3<<T as Div<A::Output>>::Output>
    where
        T: Mul<Output = A> + Copy + Div<A::Output>,
        A::Output: Float,
        A: Add<Output = A>,
    {
        self / self.magnitude()
    }

    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Vec3<U> {
        Vec3 {
            dx: f(self.dx),
            dy: f(self.dy),
            dz: f(self.dz),
        }
    }

    pub fn into_vec2(self) -> Vec2<T> {
        Vec2::new(self.dx, self.dy)
    }

    pub fn into_vec4(self) -> Vec4<T>
    where
        T: Zero,
    {
        Vec4::new(self.dx, self.dy, self.dz, T::zero())
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vec3<RHS>> for Vec3<T> {
    type Output = Vec3<Output>;
    fn add(self, rhs: Vec3<RHS>) -> Self::Output {
        Vec3::new(self.dx + rhs.dx, self.dy + rhs.dy, self.dz + rhs.dz)
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vec3<RHS>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<RHS>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
        self.dz += rhs.dz
    }
}

impl<T: Mul<RHS, Output = Output>, RHS: Copy, Output> Mul<RHS> for Vec3<T> {
    type Output = Vec3<Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Vec3 {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
            dz: self.dz * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Vec3<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.dx *= rhs;
        self.dy *= rhs;
        self.dz *= rhs
    }
}

impl<T: Div<RHS, Output = Output>, RHS: Copy, Output> Div<RHS> for Vec3<T> {
    type Output = Vec3<Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Vec3 {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
            dz: self.dz / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Vec3<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.dx /= rhs;
        self.dy /= rhs;
        self.dz /= rhs
    }
}

impl<T: Rem<RHS, Output = Output>, RHS: Copy, Output> Rem<RHS> for Vec3<T> {
    type Output = Vec3<Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Vec3 {
            dx: self.dx % rhs,
            dy: self.dy % rhs,
            dz: self.dz % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Vec3<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.dx %= rhs;
        self.dy %= rhs;
        self.dz %= rhs
    }
}

impl<T: Zero> Vec3<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Vector3D<T>> for Vec3<T> {
    fn from(vector: Vector3D<T>) -> Self {
        Vec3::new(vector.x, vector.y, vector.z)
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<Vector3D<T>> for Vec3<T> {
    fn into(self) -> Vector3D<T> {
        Vector3D::new(self.dx, self.dy, self.dz)
    }
}
