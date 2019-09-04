use crate::{Max, Min, Point, Rect, Vec2};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    pub point:  Point<T>,
    pub radius: T,
}

impl<T> Circle<T> {
    pub fn new(point: Point<T>, radius: T) -> Self {
        Circle { point, radius }
    }

    pub fn bounding_rect(&self) -> Rect<<T as Sub>::Output>
    where
        T: Copy + Sub,
        T: Add<Output = <T as Sub>::Output>,
        <T as Sub>::Output: Copy + Min + Max,
    {
        let radius_offset: Vec2<T> = Vec2::new(self.radius, self.radius);
        let top_left = self.point - radius_offset;
        let bottom_right = self.point + radius_offset;
        Rect::from_points(top_left, bottom_right)
    }
}
