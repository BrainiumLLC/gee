use crate::{lerp_half, point::Point, vector::Vector};
use num_traits::One;
use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Div},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rect<T> {
    pub top:    T,
    pub left:   T,
    pub bottom: T,
    pub right:  T,
}

impl<T> Rect<T> {
    pub fn new(top: T, left: T, bottom: T, right: T) -> Rect<T> {
        Rect {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<T: Ord + Copy> Rect<T> {
    pub fn from_points(a: Point<T>, b: Point<T>) -> Self {
        Rect {
            top:    min(a.y, b.y),
            left:   min(a.x, b.x),
            bottom: max(a.y, b.y),
            right:  max(a.x, b.x),
        }
    }
}

impl<T: Copy + One + Add<Output = U>, U: Div> Rect<T> {
    pub fn center_x(&self) -> U::Output {
        lerp_half(self.left, self.right)
    }

    pub fn center_y(&self) -> U::Output {
        lerp_half(self.top, self.bottom)
    }

    pub fn center(&self) -> Point<U::Output> {
        Point::new(self.center_x(), self.center_y())
    }
}

impl<T: Copy + Ord + One + Add<Output = U>, U: Div<Output = T>> Rect<T> {
    pub fn top_center(&self) -> Point<T> {
        Point::new(self.center_x(), self.top)
    }
    pub fn bottom_center(&self) -> Point<T> {
        Point::new(self.center_x(), self.bottom)
    }
    pub fn center_left(&self) -> Point<T> {
        Point::new(self.left, self.center_y())
    }
    pub fn center_right(&self) -> Point<T> {
        Point::new(self.right, self.center_y())
    }
}

impl<T: Ord + Copy> Rect<T> {
    pub fn top_left(&self) -> Point<T> {
        Point::new(self.left, self.top)
    }

    pub fn top_right(&self) -> Point<T> {
        Point::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> Point<T> {
        Point::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> Point<T> {
        Point::new(self.right, self.bottom)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let top = self.top.max(other.top);
        let bottom = self.bottom.min(other.bottom);
        if top > bottom {
            return None;
        }

        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        if left > right {
            return None;
        }

        Some(Rect {
            top,
            left,
            bottom,
            right,
        })
    }

    pub fn union(&self, other: &Self) -> Self {
        let top = self.top.min(other.top);
        let left = self.left.min(other.left);
        let bottom = self.bottom.max(other.bottom);
        let right = self.right.max(other.right);

        Rect {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<T: Add<RHS, Output = Output>, RHS: Copy, Output> Add<Vector<RHS>> for Rect<T> {
    type Output = Rect<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Rect {
            top:    self.top + rhs.dy,
            left:   self.left + rhs.dx,
            bottom: self.bottom + rhs.dy,
            right:  self.right + rhs.dx,
        }
    }
}

impl<T: AddAssign<RHS>, RHS: Copy> AddAssign<Vector<RHS>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vector<RHS>) {
        self.top += rhs.dy;
        self.left += rhs.dx;
        self.bottom += rhs.dy;
        self.right += rhs.dx
    }
}
