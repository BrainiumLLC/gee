use crate::Vector;
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
impl<T: en::Float> Angle<T> {
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
        Self::from_radians(degrees * T::PI() / en::cast::<T, _>(180))
    }

    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }

    /// Returns an `Angle` in the range `(-PI,PI]`.
    pub fn from_xy(x: T, y: T) -> Self {
        Self::from_radians(y.atan2(x))
    }

    /// Returns an `Angle` in the range `[-PI,PI)`.
    pub fn normalize(self) -> Self {
        let radians =
            self.radians - T::TAU() * (self.radians / T::TAU() + T::one().halved()).floor();
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

    pub fn cast<U: en::Float>(&self) -> Angle<U> {
        Angle::from_radians(en::cast(self.radians))
    }

    pub fn to_f32(self) -> Angle<f32> {
        self.cast()
    }

    pub fn to_f64(self) -> Angle<f64> {
        self.cast()
    }
}

impl<T: en::Float> Add for Angle<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians + rhs.radians)
    }
}

impl<T: en::Float> AddAssign for Angle<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: en::Float> Sub for Angle<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians - rhs.radians)
    }
}

impl<T: en::Float> SubAssign for Angle<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: en::Float> Mul<T> for Angle<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians * rhs)
    }
}

impl<T: en::Float> MulAssign<T> for Angle<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: en::Float> Div<T> for Angle<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Angle::from_radians(self.radians / rhs)
    }
}

impl<T: en::Float> DivAssign<T> for Angle<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: en::Float> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Angle::from_radians(-self.radians)
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
