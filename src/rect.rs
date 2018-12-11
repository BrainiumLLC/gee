use crate::{lerp::lerp, point::Point, scalar::Scalar};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rect<T, Unit> {
    a: Point<T, Unit>,
    b: Point<T, Unit>,
}

impl<T, Unit> Rect<T, Unit> {
    pub fn new(a: Point<T, Unit>, b: Point<T, Unit>) -> Self {
        Rect { a, b }
    }
}

impl<T: PartialOrd, Unit> Rect<T, Unit> {
    pub fn points(&self) -> [&Point<T, Unit>; 2] {
        [&self.a, &self.b]
    }
}

impl<T: PartialOrd + Clone, Unit> Rect<T, Unit> {
    pub fn top(&self) -> Scalar<T, Unit> {
        Ord::min(self.a.y().clone(), self.b.y().clone())
    }

    pub fn bottom(&self) -> Scalar<T, Unit> {
        Ord::max(self.a.y().clone(), self.b.y().clone())
    }

    pub fn left(&self) -> Scalar<T, Unit> {
        Ord::min(self.a.x().clone(), self.b.x().clone())
    }

    pub fn right(&self) -> Scalar<T, Unit> {
        Ord::max(self.a.x().clone(), self.b.x().clone())
    }

    pub fn top_left(&self) -> Point<T, Unit> {
        Point::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Point<T, Unit> {
        Point::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Point<T, Unit> {
        Point::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Point<T, Unit> {
        Point::new(self.right(), self.bottom())
    }
}

impl<T, U, V, Unit> Rect<T, Unit>
where
    T: Sub<T, Output = U> + Add<V, Output = T> + Clone,
    U: Mul<f32, Output = V>,
{
    pub fn center_x(&self) -> Scalar<T, Unit> {
        lerp(self.a.x().clone(), self.b.x().clone(), 0.5)
    }

    pub fn center_y(&self) -> Scalar<T, Unit> {
        lerp(self.a.y().clone(), self.b.y().clone(), 0.5)
    }

    pub fn center(&self) -> Point<T, Unit> {
        Point::new(self.center_x(), self.center_y())
    }
}

impl<T, U, V, Unit> Rect<T, Unit>
where
    T: Sub<T, Output = U> + Add<V, Output = T> + Clone + PartialOrd,
    U: Mul<f32, Output = V>,
{
    pub fn center_left(&self) -> Point<T, Unit> {
        Point::new(self.left(), self.center_y())
    }
    pub fn center_right(&self) -> Point<T, Unit> {
        Point::new(self.right(), self.center_y())
    }
    pub fn top_center(&self) -> Point<T, Unit> {
        Point::new(self.center_x(), self.top())
    }
    pub fn bottom_center(&self) -> Point<T, Unit> {
        Point::new(self.center_x(), self.bottom())
    }
}

impl<T: PartialOrd + Clone, Unit> Rect<T, Unit> {
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let max_top = self.top().max(other.top());
        let max_left = self.left().max(other.left());
        let min_bottom = self.bottom().min(other.bottom());
        let min_right = self.right().min(other.right());

        if max_top <= min_bottom && max_left <= min_right {
            Some(Rect::new(
                Point::new(max_left, max_top),
                Point::new(min_right, min_bottom),
            ))
        } else {
            None
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let min_top = self.top().min(other.top());
        let min_left = self.left().min(other.left());
        let max_bottom = self.bottom().max(other.bottom());
        let max_right = self.right().max(other.right());

        Rect::new(
            Point::new(min_left, min_top),
            Point::new(max_right, max_bottom),
        )
    }
}
