use crate::{OrdinaryFloat, OrdinaryNum, Vector, Vec3};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec4<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
    pub dw: T,
}

impl<T: OrdinaryNum> Vec4<T> {
    pub fn new(dx: T, dy: T, dz: T, dw: T) -> Self {
        Vec4 { dx, dy, dz, dw }
    }

    pub fn from_vector(vector: Vector<T>) -> Self {
        Self::new(vector.dx, vector.dy, T::zero(), T::zero())
    }

    pub fn from_vec3(vec3: Vec3<T>) -> Self {
        Self::new(vec3.dx, vec3.dy, vec3.dz, T::zero())
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    pub fn dot_product(self, rhs: Self) -> T {
        self.dx * rhs.dx + self.dy * rhs.dy + self.dz * rhs.dz + self.dw * rhs.dw
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

    pub fn normalized(self) -> Vec4<T>
    where
        T: OrdinaryFloat,
    {
        self / self.magnitude()
    }

    pub fn map<U: OrdinaryNum>(self, mut f: impl FnMut(T) -> U) -> Vec4<U> {
        Vec4::new(f(self.dx), f(self.dy), f(self.dz), f(self.dw))
    }

    impl_casts_and_cast!(Vec4);

    pub fn to_vector(self) -> Vector<T> {
        Vector::new(self.dx, self.dy)
    }

    pub fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.dx, self.dy, self.dz)
    }
}

impl<T: OrdinaryNum> Add for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4::new(
            self.dx + rhs.dx,
            self.dy + rhs.dy,
            self.dz + rhs.dz,
            self.dw + rhs.dw,
        )
    }
}

impl<T: OrdinaryFloat> AddAssign<Self> for Vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Vec4<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Vec4<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}
