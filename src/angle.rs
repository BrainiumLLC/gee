use crate::{cast, OrdinaryFloat, Vector};
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
impl<T: OrdinaryFloat> Angle<T> {
    pub fn ZERO() -> Self {
        Self::from_radians(T::zero())
    }

    pub fn FRAC_PI_2() -> Self {
        Self::from_radians(T::FRAC_PI_2())
    }

    pub fn FRAC_3PI_2() -> Self {
        Self::from_radians(T::FRAC_PI_2() * T::three())
    }

    pub fn FRAC_PI_3() -> Self {
        Self::from_radians(T::FRAC_PI_3())
    }

    pub fn FRAC_PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4())
    }

    pub fn FRAC_3PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::three())
    }

    pub fn FRAC_5PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::five())
    }

    pub fn FRAC_7PI_4() -> Self {
        Self::from_radians(T::FRAC_PI_4() * T::seven())
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
        Self::from_radians(T::PI()) * T::two()
    }

    pub fn from_degrees(degrees: T) -> Self {
        Angle::from_radians(degrees * T::PI() / cast::num::<T, _>(180))
    }

    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }

    /// Returns an `Angle` in the range `(-PI,PI]`.
    pub fn from_xy(x: T, y: T) -> Self {
        Angle::from_radians(y.atan2(x))
    }

    /// Returns an angle in the range [-PI,PI).
    pub fn normalize(self) -> Self {
        let tau = Self::TAU().radians;
        let radians = self.radians - tau * ((self.radians + Self::PI().radians) / tau).floor();
        Self::from_radians(radians)
    }

    pub fn radians(self) -> T {
        self.radians
    }

    pub fn unit_vector(self) -> Vector<T> {
        let (y, x) = self.radians.sin_cos();
        Vector::new(x, y)
    }

    pub fn sin(self) -> T {
        self.radians.sin()
    }

    pub fn cos(self) -> T {
        self.radians.cos()
    }

    pub fn sin_cos(self) -> (T, T) {
        self.radians.sin_cos()
    }

    pub fn tan(self) -> T {
        self.radians.tan()
    }

    pub fn cast<U: OrdinaryFloat>(&self) -> Angle<U> {
        Angle::from_radians(cast::num(self.radians))
    }

    pub fn to_f32(self) -> Angle<f32> {
        self.cast()
    }

    pub fn to_f64(self) -> Angle<f64> {
        self.cast()
    }
}

impl<T: OrdinaryFloat> Add for Angle<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians + rhs.radians)
    }
}

impl<T: OrdinaryFloat> AddAssign for Angle<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryFloat> Sub for Angle<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians - rhs.radians)
    }
}

impl<T: OrdinaryFloat> SubAssign for Angle<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: OrdinaryFloat> Mul<T> for Angle<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians * rhs)
    }
}

impl<T: OrdinaryFloat> MulAssign<T> for Angle<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: OrdinaryFloat> Div<T> for Angle<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians / rhs)
    }
}

impl<T: OrdinaryFloat> DivAssign<T> for Angle<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: OrdinaryFloat> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Angle::from_radians(-self.radians)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryFloat> From<euclid::Angle<T>> for Angle<T> {
    fn from(angle: euclid::Angle<T>) -> Self {
        Angle::from_radians(angle.radians)
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryFloat> Into<euclid::Angle<T>> for Angle<T> {
    fn into(self) -> euclid::Angle<T> {
        euclid::Angle::radians(self.radians)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn normalize() {
        let x = Angle::from_radians(3.1).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(3.1).radians);
        let x = Angle::from_radians(-3.1).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(-3.1).radians);
        let x = Angle::from_radians(0.1).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(0.1).radians);
        let x = Angle::from_radians(-0.1).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(-0.1).radians);
        let x = Angle::from_radians(0.0).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(0.0).radians);
        let x = (-Angle::PI()).normalize();
        assert_approx_eq!(x.radians, -Angle::<f32>::PI().radians);
        let x = Angle::PI().normalize();
        assert_approx_eq!(x.radians, -Angle::<f32>::PI().radians);

        let x = Angle::from_radians(3.2).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(-3.0831854).radians);
        let x = Angle::from_radians(-3.2).normalize();
        assert_approx_eq!(x.radians, Angle::from_radians(3.0831854).radians);
    }
}
