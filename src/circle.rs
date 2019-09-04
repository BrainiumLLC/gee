use crate::{Angle, Max, Min, Point, Rect, Vec2};
use num_traits::{Float, FloatConst, NumCast, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T: NumCast + Zero> Default for Circle<T> {
    fn default() -> Self {
        Self::unit()
    }
}

impl<T> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Self {
        Circle { center, radius }
    }

    pub fn unit() -> Self
    where
        T: NumCast + Zero,
    {
        Self::with_radius(T::from(1.0).unwrap())
    }

    pub fn with_radius(radius: T) -> Self
    where
        T: Zero,
    {
        Self::new(Point::zero(), radius)
    }

    pub fn with_center(center: Point<T>) -> Self
    where
        T: NumCast,
    {
        Self::new(center, T::from(1.0).unwrap())
    }

    pub fn bounding_rect(&self) -> Rect<<T as Sub>::Output>
    where
        T: Copy + Sub,
        T: Add<Output = <T as Sub>::Output>,
        <T as Sub>::Output: Copy + Min + Max,
    {
        let radius_offset: Vec2<T> = Vec2::new(self.radius, self.radius);
        let top_left = self.center - radius_offset;
        let bottom_right = self.center + radius_offset;
        Rect::from_points(top_left, bottom_right)
    }

    pub fn arc_points(
        &self,
        steps: u32,
        start_angle: Angle<T>,
        end_angle: Angle<T>,
    ) -> impl Iterator<Item = Point<T>> + Clone
    where
        T: Float + FloatConst,
    {
        let radius = self.radius;
        let center = self.center;
        let steps_float = T::from(steps).unwrap();
        let increment = (end_angle - start_angle) / steps_float;
        (0..steps).map(move |index| {
            let unit = (increment * T::from(index).unwrap() + start_angle).unit_vector();
            center + unit * radius
        })
    }

    pub fn circle_points(
        &self,
        steps: u32,
        start_angle: Angle<T>,
    ) -> impl Iterator<Item = Point<T>> + Clone
    where
        T: Float + FloatConst,
    {
        self.arc_points(steps, start_angle, start_angle + Angle::TAU())
    }
}
