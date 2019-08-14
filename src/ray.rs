use crate::{LineSegment, Point, Vec2};
use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Ray<T> {
    pub point: Point<T>,
    pub angle: T,
}

impl<T> Ray<T> {
    pub fn new(point: Point<T>, angle: T) -> Self {
        Ray { point, angle }
    }
}

impl<
        T: Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + PartialOrd
            + From<u8>
            + Float,
    > Ray<T>
where
    Vec2<T>: Mul<T, Output = Vec2<T>>,
{
    pub fn intersection(&self, other: Self) -> Option<Point<T>> {
        // adapted from https://stackoverflow.com/a/2932601
        let d = other.point - self.point;
        let self_unit = self.unit_vector();
        let other_unit = other.unit_vector();
        let det = other_unit.dx * self_unit.dy - other_unit.dy * self_unit.dx;
        let u = (d.dy * other_unit.dx - d.dx * other_unit.dy) / det;
        let v = (d.dy * self_unit.dx - d.dx * self_unit.dy) / det;
        if u >= 0.into() && v >= 0.into() {
            Some(self.point + self_unit * u)
        } else {
            None
        }
    }

    pub fn line_segment_intersection(&self, line_segment: LineSegment<T>) -> Option<Point<T>> {
        self.intersection(line_segment.ray())
            .filter(|intersection| {
                line_segment.vector().magnitude_squared()
                    <= (*intersection - self.point).magnitude_squared()
            })
    }
}

impl<T: Float> Ray<T> {
    pub fn unit_vector(&self) -> Vec2<T> {
        Vec2::unit_from_angle(self.angle)
    }
}
