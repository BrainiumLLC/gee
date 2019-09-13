use crate::{vec2::Vec2, vec3::Vec3};
use num_traits::{Float, Zero};
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

impl<T> Vec4<T> {
    pub fn new(dx: T, dy: T, dz: T, dw: T) -> Self {
        Vec4 { dx, dy, dz, dw }
    }

    pub fn from_vec2(vec2: Vec2<T>) -> Self
    where
        T: Zero,
    {
        Self::new(vec2.dx, vec2.dy, Zero::zero(), Zero::zero())
    }

    pub fn from_vec3(vec3: Vec3<T>) -> Self
    where
        T: Zero,
    {
        Self::new(vec3.dx, vec3.dy, vec3.dz, Zero::zero())
    }

    pub fn as_ref(&self) -> Vec4<&T> {
        Vec4 {
            dx: &self.dx,
            dy: &self.dy,
            dz: &self.dz,
            dw: &self.dw,
        }
    }

    pub fn as_mut(&mut self) -> Vec4<&mut T> {
        Vec4 {
            dx: &mut self.dx,
            dy: &mut self.dy,
            dz: &mut self.dz,
            dw: &mut self.dw,
        }
    }

    pub fn dot_product<RHS, A>(self, rhs: Vec4<RHS>) -> A::Output
    where
        T: Mul<RHS, Output = A>,
        A: Add<Output = A>,
    {
        self.dx * rhs.dx + self.dy * rhs.dy + self.dz * rhs.dz + self.dw * rhs.dw
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

    pub fn normalized<A>(self) -> Vec4<<T as Div<A::Output>>::Output>
    where
        T: Mul<Output = A> + Copy + Div<A::Output>,
        A::Output: Float,
        A: Add<Output = A>,
    {
        self / self.magnitude()
    }

    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Vec4<U> {
        Vec4 {
            dx: f(self.dx),
            dy: f(self.dy),
            dz: f(self.dz),
            dw: f(self.dw),
        }
    }

    pub fn into_vec2(self) -> Vec2<T> {
        Vec2::new(self.dx, self.dy)
    }

    pub fn into_vec3(self) -> Vec3<T> {
        Vec3::new(self.dx, self.dy, self.dz)
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vec4<RHS>> for Vec4<T> {
    type Output = Vec4<Output>;
    fn add(self, rhs: Vec4<RHS>) -> Self::Output {
        Vec4::new(
            self.dx + rhs.dx,
            self.dy + rhs.dy,
            self.dz + rhs.dz,
            self.dw + rhs.dw,
        )
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vec4<RHS>> for Vec4<T> {
    fn add_assign(&mut self, rhs: Vec4<RHS>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
        self.dz += rhs.dz;
        self.dw += rhs.dw
    }
}

impl<T: Mul<RHS, Output = Output>, RHS: Copy, Output> Mul<RHS> for Vec4<T> {
    type Output = Vec4<Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Vec4 {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
            dz: self.dz * rhs,
            dw: self.dw * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Vec4<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.dx *= rhs;
        self.dy *= rhs;
        self.dz *= rhs;
        self.dw *= rhs
    }
}

impl<T: Div<RHS, Output = Output>, RHS: Copy, Output> Div<RHS> for Vec4<T> {
    type Output = Vec4<Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Vec4 {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
            dz: self.dz / rhs,
            dw: self.dw / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Vec4<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.dx /= rhs;
        self.dy /= rhs;
        self.dz /= rhs;
        self.dw /= rhs
    }
}

impl<T: Rem<RHS, Output = Output>, RHS: Copy, Output> Rem<RHS> for Vec4<T> {
    type Output = Vec4<Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Vec4 {
            dx: self.dx % rhs,
            dy: self.dy % rhs,
            dz: self.dz % rhs,
            dw: self.dw % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Vec4<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.dx %= rhs;
        self.dy %= rhs;
        self.dz %= rhs;
        self.dw %= rhs
    }
}

impl<T: Zero> Vec4<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}
