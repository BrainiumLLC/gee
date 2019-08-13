use crate::{Point, Ray, Vec2};
use num_traits::Float;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct LineSegment<T> {
    pub from: Point<T>,
    pub to:   Point<T>,
}

impl<T> LineSegment<T> {
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Self { from, to }
    }
}

impl<T: Copy> LineSegment<T>
where
    Point<T>: Sub<Output = Vec2<T>>,
{
    pub fn vector(&self) -> Vec2<T> {
        self.from - self.to
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
