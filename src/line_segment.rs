use crate::{Point, Ray, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct LineSegment<T> {
    pub from: Point<T>,
    pub to:   Point<T>,
}

impl<T: en::Num> LineSegment<T> {
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Self { from, to }
    }

    pub fn length(&self) -> T
    where
        T: en::Float,
    {
        (self.to - self.from).magnitude()
    }

    pub fn map<U: en::Num>(self, mut f: impl FnMut(Point<T>) -> Point<U>) -> LineSegment<U> {
        LineSegment::new(f(self.from), f(self.to))
    }

    pub fn vector(&self) -> Vector<T> {
        self.to - self.from
    }

    pub fn ray(&self) -> Ray<T>
    where
        T: en::Float,
    {
        Ray::new(self.from, self.vector().angle())
    }
}
