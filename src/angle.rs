use crate::Vec2;
use num_traits::{Float, FloatConst};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Angle<T> {
    radians: T,
}

#[allow(non_snake_case)]
impl<T: FloatConst + Float> Angle<T> {
    pub fn ZERO() -> Self {
        Self::from_radians(T::from(0).unwrap())
    }

    pub fn FRAC_PI_2() -> Self {
        Self::from_radians(T::FRAC_PI_2())
    }

    pub fn FRAC_3PI_2() -> Self {
        Self::from_radians(T::FRAC_PI_2() * T::from(3).unwrap())
    }

    pub fn FRAC_PI_3() -> Self {
        Self::from_radians(T::FRAC_PI_3())
    }

    pub fn FRAC_PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4())
    }

    pub fn FRAC_3PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::from(3).unwrap())
    }

    pub fn FRAC_5PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::from(5).unwrap())
    }

    pub fn FRAC_7PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::from(7).unwrap())
    }

    pub fn FRAC_PI_6() -> Self {
        Self::from_radians(T::FRAC_PI_6())
    }

    pub fn FRAC_PI_8() -> Self {
        Self::from_radians(T::FRAC_PI_8())
    }

    pub fn PI() -> Self {
        Self::from_radians(T::PI())
    }

    pub fn TAU() -> Self {
        Self::from_radians(T::PI()) * T::from(2).unwrap()
    }

    pub fn from_degrees(degrees: T) -> Self {
        Angle::from_radians(degrees * T::PI() / T::from(180).unwrap())
    }
}

impl<T> Angle<T> {
    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }
}

impl<T: Float> Angle<T> {
    /// Returns an `Angle` in the range `(-PI,PI]`.
    pub fn from_xy(x: T, y: T) -> Self {
        Angle::from_radians(y.atan2(x))
    }

    pub fn radians(&self) -> T {
        self.radians
    }

    pub fn unit_vector(&self) -> Vec2<T> {
        let (y, x) = self.radians.sin_cos();
        Vec2::new(x, y)
    }

    pub fn sin(&self) -> T {
        self.radians.sin()
    }

    pub fn cos(&self) -> T {
        self.radians.cos()
    }

    pub fn tan(&self) -> T {
        self.radians.tan()
    }
}

impl<T: Add> Add for Angle<T> {
    type Output = Angle<T::Output>;

    fn add(self, rhs: Angle<T>) -> Self::Output {
        Angle::from_radians(self.radians + rhs.radians)
    }
}

impl<T: AddAssign> AddAssign for Angle<T> {
    fn add_assign(&mut self, rhs: Angle<T>) {
        self.radians += rhs.radians
    }
}

impl<T: Sub> Sub for Angle<T> {
    type Output = Angle<T::Output>;

    fn sub(self, rhs: Angle<T>) -> Self::Output {
        Angle::from_radians(self.radians - rhs.radians)
    }
}

impl<T: SubAssign> SubAssign for Angle<T> {
    fn sub_assign(&mut self, rhs: Angle<T>) {
        self.radians -= rhs.radians
    }
}

impl<T: Mul> Mul<T> for Angle<T> {
    type Output = Angle<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians * rhs)
    }
}

impl<T: MulAssign> MulAssign<T> for Angle<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.radians *= rhs
    }
}

impl<T: Div> Div<T> for Angle<T> {
    type Output = Angle<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians / rhs)
    }
}

impl<T: DivAssign> DivAssign<T> for Angle<T> {
    fn div_assign(&mut self, rhs: T) {
        self.radians /= rhs
    }
}

impl<T: Neg> Neg for Angle<T> {
    type Output = Angle<T::Output>;

    fn neg(self) -> Self::Output {
        Angle::from_radians(-self.radians)
    }
}

#[cfg(feature = "euclid")]
impl<T> From<euclid::Angle<T>> for Angle<T> {
    fn from(angle: euclid::Angle<T>) -> Self {
        Angle::from_radians(angle.radians)
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<euclid::Angle<T>> for Angle<T> {
    fn into(self) -> euclid::Angle<T> {
        euclid::Angle::radians(self.radians)
    }
}
