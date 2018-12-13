use crate::{partial_max, partial_min, point::Point, vector::Vector};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rect<T> {
    a: Point<T>,
    b: Point<T>,
}

impl<T> Rect<T> {
    pub fn new(a: Point<T>, b: Point<T>) -> Self {
        Rect { a, b }
    }
}

impl<T: PartialOrd + Clone> Rect<T> {
    pub fn top(&self) -> T {
        partial_min(self.a.y().clone(), self.b.y().clone()).unwrap()
    }

    pub fn bottom(&self) -> T {
        partial_max(self.a.y().clone(), self.b.y().clone()).unwrap()
    }

    pub fn left(&self) -> T {
        partial_min(self.a.x().clone(), self.b.x().clone()).unwrap()
    }

    pub fn right(&self) -> T {
        partial_max(self.a.x().clone(), self.b.x().clone()).unwrap()
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
        let max_top = partial_max(self.top(), other.top()).unwrap();
        let min_bottom = partial_min(self.bottom(), other.bottom()).unwrap();
        if max_top > min_bottom {
            return None;
        }

        let max_left = partial_max(self.left(), other.left()).unwrap();
        let min_right = partial_min(self.right(), other.right()).unwrap();
        if max_left > min_right {
            return None;
        }

        Some(Rect::new(
            Point::new(max_left, max_top),
            Point::new(min_right, min_bottom),
        ))
    }

    pub fn union(&self, other: &Self) -> Self {
        let min_top = partial_min(self.top(), other.top()).unwrap();
        let min_left = partial_min(self.left(), other.left()).unwrap();
        let max_bottom = partial_max(self.bottom(), other.bottom()).unwrap();
        let max_right = partial_max(self.right(), other.right()).unwrap();

        Rect::new(
            Point::new(min_left, min_top),
            Point::new(max_right, max_bottom),
        )
    }
}

impl<T: Add<RHS, Output = Output>, RHS: Clone, Output> Add<Vector<RHS>> for Rect<T> {
    type Output = Rect<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Rect::new(self.a + rhs.clone(), self.b + rhs)
    }
}

impl<'a, T: Add<&'a RHS, Output = Output>, RHS, Output> Add<&'a Vector<RHS>> for Rect<T> {
    type Output = Rect<Output>;
    fn add(self, rhs: &'a Vector<RHS>) -> Self::Output {
        Rect::new(self.a + rhs, self.b + rhs)
    }
}

impl<'a, T, RHS: Clone, Output> Add<Vector<RHS>> for &'a Rect<T>
where
    &'a T: Add<RHS, Output = Output>,
{
    type Output = Rect<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Rect::new(&self.a + rhs.clone(), &self.b + rhs)
    }
}

impl<'a, 'b, T, RHS, Output> Add<&'b Vector<RHS>> for &'a Rect<T>
where
    &'a T: Add<&'b RHS, Output = Output>,
{
    type Output = Rect<Output>;
    fn add(self, rhs: &'b Vector<RHS>) -> Self::Output {
        Rect::new(&self.a + rhs, &self.b + rhs)
    }
}
