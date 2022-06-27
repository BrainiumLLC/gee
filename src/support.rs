use crate::{Point, Rect, Vector, AABB};
use itertools::{Itertools, MinMaxResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    /// Computes the axis-aligned bounding box of the quad.
    pub fn aabb(&self) -> AABB<T> {
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

/// An intermediate type that gets produced when transforming a `Vector` by a `Transform3d`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Vector4d<T = f32> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
    pub dw: T,
}

impl<T: en::Num> Vector4d<T> {
    pub fn truncate(self) -> Vector<T> {
        Vector::new(self.dx, self.dy)
    }
}
