use crate::{Angle, LineSegment, OrdinaryFloat, OrdinaryNum, Point, Vec2};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Ray<T> {
    pub point: Point<T>,
    pub angle: Angle<T>,
}

impl<T: OrdinaryNum> Ray<T> {
    pub fn new(point: Point<T>, angle: Angle<T>) -> Self {
        Ray { point, angle }
    }

    pub fn intersection(&self, other: Self) -> Option<Point<T>>
    where
        T: OrdinaryFloat,
    {
        // adapted from https://stackoverflow.com/a/2932601
        let d = other.point - self.point;
        let self_unit = self.unit_vec2();
        let other_unit = other.unit_vec2();
        let det = other_unit.dx * self_unit.dy - other_unit.dy * self_unit.dx;
        let u = (d.dy * other_unit.dx - d.dx * other_unit.dy) / det;
        let v = (d.dy * self_unit.dx - d.dx * self_unit.dy) / det;
        if u >= T::zero() && v >= T::zero() {
            Some(self.point + self_unit * u)
        } else {
            None
        }
    }

    pub fn line_segment_intersection(&self, line_segment: LineSegment<T>) -> Option<Point<T>>
    where
        T: OrdinaryFloat,
    {
        self.intersection(line_segment.ray())
            .filter(|intersection| {
                line_segment.vec2().magnitude_squared()
                    <= (*intersection - self.point).magnitude_squared()
            })
    }

    pub fn unit_vec2(&self) -> Vec2<T>
    where
        T: OrdinaryFloat,
    {
        self.angle.unit_vec2()
    }
}
