use crate::{point::Point, scalar::Scalar};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rectangle<T, Unit> {
    a: Point<T, Unit>,
    b: Point<T, Unit>,
}

impl<T: PartialOrd, Unit> Rectangle<T, Unit> {
    pub fn points(&self) -> [&Point<T, Unit>; 2] {
        [&self.a, &self.b]
    }
}

impl<T: Clone, Unit> Rectangle<T, Unit> {
    fn select<Fx, Fy>(&self, fx: Fx, fy: Fy) -> Point<T, Unit>
    where
        Fx: FnOnce(Scalar<T, Unit>, Scalar<T, Unit>) -> Scalar<T, Unit>,
        Fy: FnOnce(Scalar<T, Unit>, Scalar<T, Unit>) -> Scalar<T, Unit>,
    {
        Point::new(
            fx(self.a.x().clone(), self.b.x().clone()),
            fy(self.a.y().clone(), self.b.y().clone()),
        )
    }

    fn partial_select<Fx, Fy>(&self, fx: Fx, fy: Fy) -> Option<Point<T, Unit>>
    where
        Fx: FnOnce(Scalar<T, Unit>, Scalar<T, Unit>) -> Option<Scalar<T, Unit>>,
        Fy: FnOnce(Scalar<T, Unit>, Scalar<T, Unit>) -> Option<Scalar<T, Unit>>,
    {
        Some(Point::new(
            fx(self.a.x().clone(), self.b.x().clone())?,
            fy(self.a.y().clone(), self.b.y().clone())?,
        ))
    }
}

impl<T: Ord + Clone, Unit> Rectangle<T, Unit> {
    pub fn top_left(&self) -> Point<T, Unit> {
        self.select(Ord::min, Ord::min)
    }

    pub fn top_right(&self) -> Point<T, Unit> {
        self.select(Ord::max, Ord::min)
    }

    pub fn bottom_left(&self) -> Point<T, Unit> {
        self.select(Ord::min, Ord::max)
    }

    pub fn bottom_right(&self) -> Point<T, Unit> {
        self.select(Ord::max, Ord::max)
    }
}

fn partial_min<T: PartialOrd>(a: T, b: T) -> Option<T> {
    Some(match a.partial_cmp(&b)? {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    })
}

fn partial_max<T: PartialOrd>(a: T, b: T) -> Option<T> {
    Some(match a.partial_cmp(&b)? {
        Ordering::Greater => a,
        Ordering::Less | Ordering::Equal => b,
    })
}

impl<T: PartialOrd + Clone, Unit> Rectangle<T, Unit> {
    pub fn partial_top_left(&self) -> Option<Point<T, Unit>> {
        self.partial_select(partial_min, partial_min)
    }

    pub fn partial_top_right(&self) -> Option<Point<T, Unit>> {
        self.partial_select(partial_max, partial_min)
    }

    pub fn partial_bottom_left(&self) -> Option<Point<T, Unit>> {
        self.partial_select(partial_min, partial_max)
    }

    pub fn partial_bottom_right(&self) -> Option<Point<T, Unit>> {
        self.partial_select(partial_max, partial_max)
    }
}
