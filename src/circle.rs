use crate::{Angle, OrdinaryNum, Point, Rect, Vec2};
use num_traits::{Float, FloatConst};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    center: Point<T>,
    radius: T,
}

impl<T: OrdinaryNum> Default for Circle<T> {
    fn default() -> Self {
        Self::unit()
    }
}

impl<T: OrdinaryNum> Circle<T> {
    pub fn new_unchecked(center: Point<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn try_new(center: Point<T>, radius: T) -> Option<Self> {
        if radius >= T::zero() {
            Some(Self::new_unchecked(center, radius))
        } else {
            None
        }
    }

    pub fn new(center: Point<T>, radius: T) -> Self {
        Self::try_new(center, radius).expect("radius is less than 0")
    }

    pub fn unit() -> Self {
        Self::with_radius(T::one())
    }

    pub fn zero() -> Self {
        Self::with_radius(T::zero())
    }

    pub fn with_radius(radius: T) -> Self {
        Self::new(Point::zero(), radius)
    }

    pub fn with_center(center: Point<T>) -> Self {
        Self::new(center, T::one())
    }

    pub fn bounding_rect(&self) -> Rect<T> {
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
