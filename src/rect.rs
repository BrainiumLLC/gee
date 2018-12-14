use crate::{point::Point, vector::Vector};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect<T> {
    a: Point<T>,
    b: Point<T>,
}

impl<T> Rect<T> {
    pub fn new(a: Point<T>, b: Point<T>) -> Self {
        Rect { a, b }
    }
}

impl<T: PartialOrd + PartialEq + Ord + Clone> Ord for Rect<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.top_left(), &self.bottom_right()).cmp(&(&other.top_left(), &other.bottom_right()))
    }
}

impl<T: PartialOrd + PartialEq + Ord + Clone> PartialOrd for Rect<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&self.top_left(), &self.bottom_right())
            .partial_cmp(&(&other.top_left(), &other.bottom_right()))
    }
}

impl<T: Eq + PartialEq + Ord + Clone> Eq for Rect<T> {}

impl<T: PartialEq + Ord + Clone> PartialEq for Rect<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.top_left() == rhs.top_left() && self.bottom_right() == rhs.bottom_right()
    }
    fn ne(&self, rhs: &Self) -> bool {
        self.top_left() != rhs.top_left() || self.bottom_right() != rhs.bottom_right()
    }
}

impl<T: Hash + Ord + Clone> Hash for Rect<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.top_left().hash(state);
        self.bottom_right().hash(state)
    }
}

impl<T: Ord + Clone> Rect<T> {
    pub fn top(&self) -> T {
        self.a.y().clone().min(self.b.y().clone())
    }

    pub fn bottom(&self) -> T {
        self.a.y().clone().max(self.b.y().clone())
    }

    pub fn left(&self) -> T {
        self.a.x().clone().min(self.b.x().clone())
    }

    pub fn right(&self) -> T {
        self.a.x().clone().max(self.b.x().clone())
    }

    pub fn top_left(&self) -> Point<T> {
        Point::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Point<T> {
        Point::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Point<T> {
        Point::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Point<T> {
        Point::new(self.right(), self.bottom())
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let max_top = self.top().max(other.top());
        let min_bottom = self.bottom().min(other.bottom());
        if max_top > min_bottom {
            return None;
        }

        let max_left = self.left().max(other.left());
        let min_right = self.right().min(other.right());
        if max_left > min_right {
            return None;
        }

        Some(Rect::new(
            Point::new(max_left, max_top),
            Point::new(min_right, min_bottom),
        ))
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

impl<T: Add<RHS, Output = Output>, RHS: Copy, Output> Add<Vector<RHS>> for Rect<T> {
    type Output = Rect<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Rect::new(self.a + rhs, self.b + rhs)
    }
}

impl<T: AddAssign<RHS>, RHS: Copy> AddAssign<Vector<RHS>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vector<RHS>) {
        self.a += rhs;
        self.b += rhs
    }
}
