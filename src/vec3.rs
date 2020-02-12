use crate::{OrdinaryFloat, OrdinaryNum, Vec2, Vec4};
#[cfg(feature = "euclid")]
use euclid::Vector3D;
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

impl<T: OrdinaryNum> Vec3<T> {
    pub fn new(dx: T, dy: T, dz: T) -> Self {
        Vec3 { dx, dy, dz }
    }

    pub fn from_vec2(vec2: Vec2<T>) -> Self {
        Self::new(vec2.dx, vec2.dy, T::zero())
    }

    pub fn from_vec4(vec4: Vec4<T>) -> Self {
        Self::new(vec4.dx, vec4.dy, vec4.dz)
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    pub fn dot_product(self, rhs: Self) -> T {
        self.dx * rhs.dx + self.dy * rhs.dy + self.dz * rhs.dz
    }

    pub fn magnitude_squared(self) -> T {
        self.dot_product(self)
    }

    pub fn magnitude(self) -> T
    where
        T: OrdinaryFloat,
    {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(self) -> Self
    where
        T: OrdinaryFloat,
    {
        self / self.magnitude()
    }

    pub fn map<U: OrdinaryNum>(self, mut f: impl FnMut(T) -> U) -> Vec3<U> {
        Vec3::new(f(self.dx), f(self.dy), f(self.dz))
    }

    impl_casts_and_cast!(Vec3);

    pub fn to_vec2(self) -> Vec2<T> {
        Vec2::new(self.dx, self.dy)
    }

    pub fn to_vec4(self) -> Vec4<T> {
        Vec4::new(self.dx, self.dy, self.dz, T::zero())
    }
}

impl<T: OrdinaryNum> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.dx + rhs.dx, self.dy + rhs.dy, self.dz + rhs.dz)
    }
}

impl<T: OrdinaryNum> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Vec3<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Vec3<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<Vector3D<T>> for Vec3<T> {
    fn from(vector: Vector3D<T>) -> Self {
        Vec3::new(vector.x, vector.y, vector.z)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<Vector3D<T>> for Vec3<T> {
    fn into(self) -> Vector3D<T> {
        Vector3D::new(self.dx, self.dy, self.dz)
    }
}
