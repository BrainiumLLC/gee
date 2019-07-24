use crate::Vec2;
use core::ops::{Div, Mul};
use num_traits::Float;

pub struct Angle<T> {
    radians: T,
}

impl<T: From<f32> + Mul<T, Output = T> + Div<T, Output = T>> Angle<T> {
    pub fn from_degrees(degrees: T) -> Self {
        Angle {
            radians: degrees * std::f32::consts::PI.into() / 180f32.into(),
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
        Vec2::new(self.radians.cos(), self.radians.sin())
    }
}
