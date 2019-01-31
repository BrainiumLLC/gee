use crate::{lerp_half, point::Point, size::Size, vector::Vector};
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::{max, min},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T> {
    pub left:   T,
    pub top:    T,
    pub right:  T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Rect<T> {
        Rect {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<T: Add<Output = T> + Copy> Rect<T> {
    pub fn with_top_left(top_left: Point<T>, size: Size<T>) -> Rect<T> {
        Rect::new(
            top_left.x,
            top_left.y,
            top_left.x + size.width,
            top_left.y + size.height,
        )
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + From<i8> + Default + Sub<Output = T> + Div<Output = T>,
{
    pub fn with_center(center: Point<T>, size: Size<T>) -> Self {
        Self::with_top_left(center - lerp_half(Size::default(), size).into(), size)
    }
}

impl<T: Copy> Rect<T> {
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
}

impl<T: Copy + Sub> Rect<T> {
    pub fn size(&self) -> Size<T::Output> {
        Size::new(self.right - self.left, self.bottom - self.top)
    }
}

impl<T: PartialOrd> Rect<T> {
    pub fn contains(&self, point: &Point<T>) -> bool {
        self.left <= point.x && point.x < self.right && self.top <= point.y && point.y < self.bottom
    }
}

impl<T: Zero> Rect<T> {
    pub fn zero() -> Self {
        Rect::new(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }

    pub fn from_size(size: Size<T>) -> Self {
        Rect::new(Zero::zero(), Zero::zero(), size.width, size.height)
    }
}

impl<T: PartialEq> Rect<T> {
    pub fn is_empty(&self) -> bool {
        self.top == self.bottom || self.left == self.bottom
    }
}

impl<T: Copy + PartialOrd + Zero> Rect<T> {
    pub fn from_points_iter<I>(points: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point<T>>,
    {
        let mut points = points.into_iter();

        let (mut min_x, mut min_y) = match points.next() {
            Some(first) => (first.borrow().x, first.borrow().y),
            None => return Rect::zero(),
        };

        let (mut max_x, mut max_y) = (min_x, min_y);
        for point in points {
            let p = point.borrow();
            if p.x < min_x {
                min_x = p.x
            }
            if p.x > max_x {
                max_x = p.x
            }
            if p.y < min_y {
                min_y = p.y
            }
            if p.y > max_y {
                max_y = p.y
            }
        }
        Rect::new(min_x, min_y, max_x, max_y)
    }
}

impl<T: Ord + Copy> Rect<T> {
    pub fn from_points(a: Point<T>, b: Point<T>) -> Self {
        Rect {
            left:   min(a.x, b.x),
            top:    min(a.y, b.y),
            right:  max(a.x, b.x),
            bottom: max(a.y, b.y),
        }
    }
}

impl<T: Copy + Add<Output = U>, U: Div + From<i8>> Rect<T> //where
{
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

impl<T: Copy + Ord + Add<Output = U>, U: Div<Output = T> + From<i8>> Rect<T> {
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

impl<T: Add<RHS>, RHS: Copy> Add<Vector<RHS>> for Rect<T> {
    type Output = Rect<T::Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Rect {
            left:   self.left + rhs.dx,
            top:    self.top + rhs.dy,
            right:  self.right + rhs.dx,
            bottom: self.bottom + rhs.dy,
        }
    }
}

impl<T: AddAssign<RHS>, RHS: Copy> AddAssign<Vector<RHS>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vector<RHS>) {
        self.left += rhs.dx;
        self.top += rhs.dy;
        self.right += rhs.dx;
        self.bottom += rhs.dy
    }
}

impl<T: Mul<RHS>, RHS: Copy> Mul<RHS> for Rect<T> {
    type Output = Rect<T::Output>;
    fn mul(self, rhs: RHS) -> Self::Output {
        Rect {
            left:   self.left * rhs,
            top:    self.top * rhs,
            right:  self.right * rhs,
            bottom: self.bottom * rhs,
        }
    }
}

impl<T: MulAssign<RHS>, RHS: Copy> MulAssign<RHS> for Rect<T> {
    fn mul_assign(&mut self, rhs: RHS) {
        self.left *= rhs;
        self.top *= rhs;
        self.right *= rhs;
        self.bottom *= rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: Add<Output = T> + Copy> From<euclid::Rect<T>> for Rect<T> {
    fn from(rect: euclid::Rect<T>) -> Self {
        Rect::with_top_left(rect.origin.into(), rect.size.into())
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy + Sub<Output = T>> Into<euclid::Rect<T>> for Rect<T> {
    fn into(self) -> euclid::Rect<T> {
        euclid::Rect::new(self.top_left().into(), self.size().into())
    }
}
