use crate::{Angle, Cardinal, Direction, OrdinaryFloat, OrdinaryNum, Point, Size, Vec3, Vec4};
#[cfg(feature = "euclid")]
use euclid::Vector2D;
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

impl<T: OrdinaryNum> Vec2<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Vec2 { dx, dy }
    }

    pub fn uniform(d: T) -> Self {
        Self::new(d, d)
    }

    pub fn from_dx(dx: T) -> Self {
        Vec2 { dx, dy: T::zero() }
    }

    pub fn from_dy(dy: T) -> Self {
        Vec2 { dx: T::zero(), dy }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    pub fn from_vec3(vec3: Vec3<T>) -> Self {
        Self::new(vec3.dx, vec3.dy)
    }

    pub fn from_vec4(vec4: Vec4<T>) -> Self {
        Self::new(vec4.dx, vec4.dy)
    }

    pub fn dot_product(self, rhs: Self) -> T {
        self.dx * rhs.dx + self.dy * rhs.dy
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

    pub fn unit_from_angle(angle: Angle<T>) -> Self
    where
        T: OrdinaryFloat,
    {
        angle.unit_vec2()
    }

    pub fn angle(self) -> Angle<T>
    where
        T: OrdinaryFloat,
    {
        Angle::from_xy(self.dx, self.dy)
    }

    pub fn scaled(self, rhs: Size<T>) -> Self {
        Self::new(self.dx * rhs.width(), self.dy * rhs.height())
    }

    pub fn perpendicular(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::new(-self.dy, self.dx)
    }

    pub fn yx(self) -> Self {
        Self::new(self.dy, self.dx)
    }

    pub fn map<U: OrdinaryNum>(&self, mut f: impl FnMut(T) -> U) -> Vec2<U> {
        Vec2::new(f(self.dx), f(self.dy))
    }

    impl_casts_and_cast!(Vec2);

    pub fn to_array(self) -> [T; 2] {
        [self.dx, self.dy]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.dx, self.dy)
    }

    pub fn to_point(self) -> Point<T> {
        Point::zero() + self
    }

    pub fn to_size(self) -> Size<T> {
        self.into()
    }

    pub fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.dx, self.dy, T::zero())
    }

    pub fn to_vec4(self) -> Vec4<T> {
        Vec4::new(self.dx, self.dy, T::zero(), T::zero())
    }
}

impl<T: OrdinaryNum> From<Size<T>> for Vec2<T> {
    fn from(size: Size<T>) -> Self {
        Vec2::new(size.width(), size.height())
    }
}

impl<T: OrdinaryNum> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<T: OrdinaryNum> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl<T: OrdinaryNum> SubAssign<Self> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Vec2<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Vec2<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<T: Neg<Output = T> + OrdinaryNum> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(move |x| -x)
    }
}

impl<T: OrdinaryNum> From<Direction> for Vec2<T> {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            North => Vec2::new(0, -1),
            East => Vec2::new(1, 0),
            South => Vec2::new(0, 1),
            West => Vec2::new(-1, 0),
            Northeast => Vec2::new(1, -1),
            Southeast => Vec2::new(1, 1),
            Southwest => Vec2::new(-1, 1),
            Northwest => Vec2::new(-1, -1),
        }
        .cast()
    }
}

impl<T: OrdinaryNum> From<Cardinal> for Vec2<T> {
    fn from(cardinal: Cardinal) -> Self {
        use Cardinal::*;
        match cardinal {
            North => Vec2::new(0, -1),
            East => Vec2::new(1, 0),
            South => Vec2::new(0, 1),
            West => Vec2::new(-1, 0),
        }
        .cast()
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<Vector2D<T>> for Vec2<T> {
    fn from(vector: Vector2D<T>) -> Self {
        Vec2::new(vector.x, vector.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<Vector2D<T>> for Vec2<T> {
    fn into(self) -> Vector2D<T> {
        Vector2D::new(self.dx, self.dy)
    }
}
