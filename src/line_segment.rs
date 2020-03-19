use crate::{Point, Ray, Vec2};
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct LineSegment<T> {
    pub from: Point<T>,
    pub to:   Point<T>,
}

impl<T> LineSegment<T> {
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Self { from, to }
    }

    pub fn length(self) -> T
    where
        T: Float,
    {
        (self.to - self.from).magnitude()
    }

    pub fn map<U>(self, mut f: impl FnMut(Point<T>) -> Point<U>) -> LineSegment<U> {
        LineSegment::new(f(self.from), f(self.to))
    }
}

impl<T: Copy> LineSegment<T>
where
    Point<T>: Sub<Output = Vec2<T>>,
{
    pub fn vector(&self) -> Vec2<T> {
        self.to - self.from
    }
}

impl<T: Float> LineSegment<T>
where
    Point<T>: Sub<Output = Vec2<T>>,
{
    pub fn ray(&self) -> Ray<T> {
        Ray::new(self.from, self.vector().angle())
    }
}
