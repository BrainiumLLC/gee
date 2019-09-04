use crate::Vec2;
use num_traits::{Float, FloatConst};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Angle<T> {
    pub radians: T,
}

#[allow(non_snake_case)]
impl<T: FloatConst + Float> Angle<T> {
    pub fn FRAC_PI_2() -> Self {
        Self::from_radians(T::FRAC_PI_2())
    }

    pub fn PI() -> Self {
        Self::from_radians(T::PI())
    }

    pub fn TAU() -> Self {
        Self::from_radians(T::PI()) * T::from(2.0).unwrap()
    }

    pub fn from_degrees(degrees: T) -> Self {
        Angle {
            radians: degrees * T::PI() / T::from(180f32).unwrap(),
        }
    }
}

impl<T> Angle<T> {
    pub fn from_radians(radians: T) -> Self {
        Angle { radians }
    }
}

impl<T: Float> Angle<T> {
    pub fn unit_vector(&self) -> Vec2<T> {
        let (y, x) = self.radians.sin_cos();
        Vec2::new(x, y)
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
