use crate::{
    direction::{Cardinal, Direction},
    size::Size,
    vec3::Vec3,
    vec4::Vec4,
};
#[cfg(feature = "euclid")]
use euclid::Vector2D;
use num_traits::{Float, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec2<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Vec2<T> {
    pub fn new<U: Into<T>>(dx: U, dy: U) -> Self {
        Vec2 {
            dx: dx.into(),
            dy: dy.into(),
        }
    }

    pub fn from_vec3(vec3: Vec3<T>) -> Self {
        Self::new(vec3.dx, vec3.dy)
    }

    pub fn from_vec4(vec4: Vec4<T>) -> Self {
        Self::new(vec4.dx, vec4.dy)
    }

    pub fn as_ref(&self) -> Vec2<&T> {
        Vec2 {
            dx: &self.dx,
            dy: &self.dy,
        }
    }

    pub fn as_mut(&mut self) -> Vec2<&mut T> {
        Vec2 {
            dx: &mut self.dx,
            dy: &mut self.dy,
        }
    }

    pub fn dot_product<RHS, A>(self, rhs: Vec2<RHS>) -> A::Output
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

    pub fn normalized<A>(self) -> Vec2<<T as Div<A::Output>>::Output>
    where
        T: Mul<Output = A> + Copy + Div<A::Output>,
        A::Output: Float,
        A: Add,
    {
        self / self.magnitude()
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2<U> {
        Vec2 {
            dx: f(self.dx),
            dy: f(self.dy),
        }
    }

    pub fn into_vec3(self) -> Vec3<T>
    where
        T: Zero,
    {
        Vec3::new(self.dx, self.dy, Zero::zero())
    }

    pub fn into_vec4(self) -> Vec4<T>
    where
        T: Zero,
    {
        Vec4::new(self.dx, self.dy, Zero::zero(), Zero::zero())
    }
}

impl<T: Float> Vec2<T> {
    pub fn unit_from_angle(angle: T) -> Self {
        Self::new(angle.cos(), angle.sin())
    }
}

impl<T: Div + Float> Vec2<T> {
    pub fn angle(self) -> T {
        (self.dy / self.dx).atan()
    }
}

impl<T> From<Size<T>> for Vec2<T> {
    fn from(size: Size<T>) -> Self {
        Vec2::new(size.width, size.height)
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vec2<RHS>> for Vec2<T> {
    type Output = Vec2<Output>;
    fn add(self, rhs: Vec2<RHS>) -> Self::Output {
        Vec2::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vec2<RHS>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<RHS>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy
    }
}

impl<T: Sub<RHS, Output = Output>, RHS, Output> Sub<Vec2<RHS>> for Vec2<T> {
    type Output = Vec2<Output>;
    fn sub(self, rhs: Vec2<RHS>) -> Self::Output {
        Vec2::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl<T: SubAssign<RHS>, RHS> SubAssign<Vec2<RHS>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<RHS>) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy
    }
}

impl<T: Mul<RHS, Output = Output>, RHS: Copy, Output> Mul<RHS> for Vec2<T> {
    type Output = Vec2<Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Vec2 {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Vec2<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.dx *= rhs;
        self.dy *= rhs
    }
}

impl<T: Div<RHS, Output = Output>, RHS: Copy, Output> Div<RHS> for Vec2<T> {
    type Output = Vec2<Output>;
    fn div(self, rhs: RHS) -> Self::Output {
        Vec2 {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T: DivAssign<RHS>, RHS: Copy> DivAssign<RHS> for Vec2<T> {
    fn div_assign(&mut self, rhs: RHS) {
        self.dx /= rhs;
        self.dy /= rhs
    }
}

impl<T: Rem<RHS, Output = Output>, RHS: Copy, Output> Rem<RHS> for Vec2<T> {
    type Output = Vec2<Output>;
    fn rem(self, rhs: RHS) -> Self::Output {
        Vec2 {
            dx: self.dx % rhs,
            dy: self.dy % rhs,
        }
    }
}

impl<T: RemAssign<RHS>, RHS: Copy> RemAssign<RHS> for Vec2<T> {
    fn rem_assign(&mut self, rhs: RHS) {
        self.dx %= rhs;
        self.dy %= rhs
    }
}

impl<T: From<i8>> From<Direction> for Vec2<T> {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            North => Self::new(0, -1),
            East => Self::new(1, 0),
            South => Self::new(0, 1),
            West => Self::new(-1, 0),
            Northeast => Self::new(1, -1),
            Southeast => Self::new(1, 1),
            Southwest => Self::new(-1, 1),
            Northwest => Self::new(-1, -1),
        }
    }
}

impl<T: From<i8>> From<Cardinal> for Vec2<T> {
    fn from(cardinal: Cardinal) -> Self {
        use Cardinal::*;
        match cardinal {
            North => Self::new(0, -1),
            East => Self::new(1, 0),
            South => Self::new(0, 1),
            West => Self::new(-1, 0),
        }
    }
}

impl<T: Neg<Output = T>> Vec2<T> {
    pub fn perpendicular(self) -> Self {
        Self::new(-self.dy, self.dx)
    }
}

impl<T: Zero> Vec2<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Vector2D<T>> for Vec2<T> {
    fn from(vector: Vector2D<T>) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<Vector2D<T>> for Vec2<T> {
    fn into(self) -> Vector2D<T> {
        Vector2D::new(self.dx, self.dy)
    }
}
