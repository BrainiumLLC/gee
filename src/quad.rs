use crate::{Point, Rect, AABB};
use itertools::{Itertools, MinMaxResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A `Quad` is a structure of 4 points that gives no specific guarantees about the relationship between them.
/// This is the output of transforming a `Rect` by an arbitrary transform, where the result might not be a `Rect`
/// anymore due to rotation/perspective distortion/etc.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Quad<T = f32> {
    pub a: Point<T>,
    pub b: Point<T>,
    pub c: Point<T>,
    pub d: Point<T>,
}

impl<T: en::Float> Quad<T> {
    fn aabb(&self) -> AABB<T> {
        let verts = [self.a, self.b, self.c, self.d];
        let (min_x, max_x) = unsafe { min_max(verts.iter().map(|v| v.x)) };
        let (min_y, max_y) = unsafe { min_max(verts.iter().map(|v| v.y)) };
        Rect::from_top_right_bottom_left(min_y, max_x, max_y, min_x).into()
    }
}

/// Safety: iter must have >1 element
unsafe fn min_max<T: PartialOrd>(i: impl IntoIterator<Item = T>) -> (T, T) {
    match i.into_iter().minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}

/// An intermediate type produced by transforming a `Rect` by a `Transform3d`.
#[derive(Debug)]
struct Quad4d<T = f32> {
    pub a: Point4d<T>,
    pub b: Point4d<T>,
    pub c: Point4d<T>,
    pub d: Point4d<T>,
}

impl<T: en::Num> Quad4d<T> {
    pub fn truncate(self) -> Quad<T> {
        Quad {
            a: self.a.truncate(),
            b: self.b.truncate(),
            c: self.c.truncate(),
            d: self.d.truncate(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point4d<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: en::Num> Point4d<T> {
    pub fn truncate(self) -> Point<T> {
        Point::new(self.x, self.y)
    }
}
