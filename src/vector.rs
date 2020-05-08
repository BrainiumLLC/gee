use crate::{Angle, Cardinal, Direction, OrdinaryFloat, OrdinaryNum, Point, Size};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T: OrdinaryNum> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }

    pub fn uniform(d: T) -> Self {
        Self::new(d, d)
    }

    pub fn from_dx(dx: T) -> Self {
        Self { dx, dy: T::zero() }
    }

    pub fn from_dy(dy: T) -> Self {
        Self { dx: T::zero(), dy }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
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
        angle.unit_vector()
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

    pub fn map<U: OrdinaryNum>(&self, mut f: impl FnMut(T) -> U) -> Vector<U> {
        Vector::new(f(self.dx), f(self.dy))
    }

    impl_casts_and_cast!(Vector);

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
}

impl<T: OrdinaryNum> From<Size<T>> for Vector<T> {
    fn from(size: Size<T>) -> Self {
        Self::new(size.width(), size.height())
    }
}

impl<T: OrdinaryNum> Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<T: OrdinaryNum> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl<T: OrdinaryNum> SubAssign<Self> for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: OrdinaryNum> Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: OrdinaryNum> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryNum> Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: OrdinaryNum> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryNum> Rem<T> for Vector<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: OrdinaryNum> RemAssign<T> for Vector<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<T: Neg<Output = T> + OrdinaryNum> Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(move |x| -x)
    }
}

impl<T: OrdinaryNum> From<Direction> for Vector<T> {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            North => Vector::new(0, -1),
            East => Vector::new(1, 0),
            South => Vector::new(0, 1),
            West => Vector::new(-1, 0),
            Northeast => Vector::new(1, -1),
            Southeast => Vector::new(1, 1),
            Southwest => Vector::new(-1, 1),
            Northwest => Vector::new(-1, -1),
        }
        .cast()
    }
}

impl<T: OrdinaryNum> From<Cardinal> for Vector<T> {
    fn from(cardinal: Cardinal) -> Self {
        use Cardinal::*;
        match cardinal {
            North => Vector::new(0, -1),
            East => Vector::new(1, 0),
            South => Vector::new(0, 1),
            West => Vector::new(-1, 0),
        }
        .cast()
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<euclid::Vector2D<T>> for Vector<T> {
    fn from(vector: euclid::Vector2D<T>) -> Self {
        Self::new(vector.x, vector.y)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<euclid::Vector2D<T>> for Vector<T> {
    fn into(self) -> euclid::Vector2D<T> {
        euclid::Vector2D::new(self.dx, self.dy)
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + OrdinaryNum> From<nalgebra_glm::TVec2<T>> for Vector<T> {
    fn from(vector: nalgebra_glm::TVec2<T>) -> Self {
        Self::new(vector.x, vector.y)
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + OrdinaryNum> Into<nalgebra_glm::TVec2<T>> for Vector<T> {
    fn into(self) -> nalgebra_glm::TVec2<T> {
        nalgebra_glm::vec2(self.dx, self.dy)
    }
}
