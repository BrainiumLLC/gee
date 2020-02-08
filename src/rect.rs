use crate::{cast, LineSegment, OrdinaryNum, Point, Size, Vec2};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: check rect validity in deserialize
pub struct Rect<T> {
    top:    T,
    right:  T,
    bottom: T,
    left:   T,
}

impl<T: OrdinaryNum> Rect<T> {
    pub fn new_unchecked(top: T, right: T, bottom: T, left: T) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn try_new(top: T, right: T, bottom: T, left: T) -> Option<Self> {
        if top <= bottom && left <= right {
            Some(Self::new_unchecked(top, right, bottom, left))
        } else {
            None
        }
    }

    pub fn new(top: T, right: T, bottom: T, left: T) -> Self {
        Self::try_new(top, right, bottom, left)
            .expect("invalid Rect (left > right and/or top > bottom)")
    }

    pub fn with_top_left(top_left: Point<T>, size: Size<T>) -> Self {
        Self::new(
            top_left.y,
            top_left.x + size.width(),
            top_left.y + size.height(),
            top_left.x,
        )
    }

    pub fn with_top_right(top_right: Point<T>, size: Size<T>) -> Self {
        Self::new(
            top_right.y,
            top_right.x,
            top_right.y + size.height(),
            top_right.x - size.width(),
        )
    }

    pub fn with_bottom_right(bottom_right: Point<T>, size: Size<T>) -> Self {
        Self::new(
            bottom_right.y - size.height(),
            bottom_right.x,
            bottom_right.y,
            bottom_right.x - size.width(),
        )
    }

    pub fn with_bottom_left(bottom_left: Point<T>, size: Size<T>) -> Self {
        Self::new(
            bottom_left.y - size.height(),
            bottom_left.x + size.width(),
            bottom_left.y,
            bottom_left.x,
        )
    }

    pub fn with_center(center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().half();
        let half_height = size.height().half();
        Self::new(
            center.y - half_height,
            center.x + half_width,
            center.y + half_height,
            center.x - half_width,
        )
    }

    pub fn with_top_center(top_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().half();
        Self::new(
            top_center.y,
            top_center.x + half_width,
            top_center.y + size.height(),
            top_center.x - half_width,
        )
    }

    pub fn with_bottom_center(bottom_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().half();
        Self::new(
            bottom_center.y - size.height(),
            bottom_center.x + half_width,
            bottom_center.y,
            bottom_center.x - half_width,
        )
    }

    pub fn with_left_center(left_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().half();
        Self::new(
            left_center.y - half_height,
            left_center.x + size.width(),
            left_center.y + half_height,
            left_center.x,
        )
    }

    pub fn with_right_center(right_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().half();
        Self::new(
            right_center.y - half_height,
            right_center.x,
            right_center.y + half_height,
            right_center.x - size.width(),
        )
    }

    pub fn split_at_x(&self, x: T) -> Option<(Self, Self)> {
        if self.contains_x(x) {
            let Self {
                top,
                right,
                bottom,
                left,
            } = *self;
            Some((
                Self::new(top, x, bottom, left),
                Self::new(top, right, bottom, x),
            ))
        } else {
            None
        }
    }

    pub fn split_at_y(&self, y: T) -> Option<(Self, Self)> {
        if self.contains_y(y) {
            let Self {
                top,
                right,
                bottom,
                left,
            } = *self;
            Some((
                Self::new(top, right, y, left),
                Self::new(y, right, bottom, left),
            ))
        } else {
            None
        }
    }

    pub fn padded(&self, top: T, right: T, bottom: T, left: T) -> Self {
        Self::new(
            self.top + top,
            self.right - right,
            self.bottom - bottom,
            self.left + left,
        )
    }

    pub fn padded_horiz_and_vert(&self, horiz: T, vert: T) -> Self {
        self.padded(vert, horiz, vert, horiz)
    }

    pub fn padded_uniform(&self, pad: T) -> Self {
        self.padded_horiz_and_vert(pad, pad)
    }

    pub fn scaled_from_top(&self, scale: T) -> Self {
        Self::new(
            self.top,
            self.right,
            self.top + self.height() * scale,
            self.left,
        )
    }

    pub fn scaled_from_bottom(&self, scale: T) -> Self {
        Self::new(
            self.bottom - self.height() * scale,
            self.right,
            self.bottom,
            self.left,
        )
    }

    pub fn scaled_from_left(&self, scale: T) -> Self {
        Self::new(
            self.top,
            self.left + self.width() * scale,
            self.bottom,
            self.left,
        )
    }

    pub fn scaled_from_right(&self, scale: T) -> Self {
        Self::new(
            self.top,
            self.right,
            self.bottom,
            self.right - self.width() * scale,
        )
    }

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

    pub fn width(&self) -> T {
        self.right - self.left
    }

    pub fn height(&self) -> T {
        self.bottom - self.top
    }

    pub fn size(&self) -> Size<T> {
        Size::new(self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> T {
        self.size().aspect_ratio()
    }

    pub fn contains_x(&self, x: T) -> bool {
        (self.left..self.right).contains(&x)
    }

    pub fn contains_y(&self, y: T) -> bool {
        (self.top..self.bottom).contains(&y)
    }

    pub fn contains(&self, point: &Point<T>) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    pub fn map<U: OrdinaryNum, F: Fn(T) -> U>(self, f: F) -> Rect<U> {
        Rect::new(f(self.top), f(self.right), f(self.bottom), f(self.left))
    }

    pub fn is_empty(&self) -> bool {
        self.top == self.bottom || self.left == self.right
    }

    pub fn line_segments(&self) -> [LineSegment<T>; 4] {
        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_right = self.bottom_right();
        let bottom_left = self.bottom_left();
        [
            LineSegment::new(top_left, top_right),
            LineSegment::new(top_right, bottom_right),
            LineSegment::new(bottom_right, bottom_left),
            LineSegment::new(bottom_left, top_left),
        ]
    }

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
        Self::new(min_y, max_x, max_y, min_x)
    }

    pub fn from_points(a: Point<T>, b: Point<T>) -> Self {
        Self::new(a.y.min(b.y), a.x.max(b.x), a.y.max(b.y), a.x.min(b.x))
    }

    pub fn center_x(&self) -> T {
        (self.left + self.right).half()
    }

    pub fn center_y(&self) -> T {
        (self.top + self.bottom).half()
    }

    pub fn center(&self) -> Point<T> {
        Point::new(self.center_x(), self.center_y())
    }

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

    pub fn width_slice(&self, num_items: usize, index: usize) -> Self {
        self.width_slice_with_margin(num_items, index, T::zero())
    }

    pub fn width_slices(self, num_items: usize) -> impl Iterator<Item = Self> {
        self.width_slices_with_margin(num_items, T::zero())
    }

    pub fn height_slice(&self, num_items: usize, index: usize) -> Self {
        self.height_slice_with_margin(num_items, index, T::zero())
    }

    pub fn height_slices(self, num_items: usize) -> impl Iterator<Item = Self> {
        self.height_slices_with_margin(num_items, T::zero())
    }

    pub fn width_slice_with_margin(&self, num_items: usize, index: usize, margin: T) -> Self {
        let num_items = cast::num(num_items);
        let index: T = cast::num(index);
        let total_margin = num_items * margin + margin;
        let items_width = self.width() - total_margin;
        let item_width = items_width / num_items;
        let item_left = self.left + margin + index * (margin + item_width);
        Rect {
            top:    self.top,
            left:   item_left,
            right:  item_left + item_width,
            bottom: self.bottom,
        }
    }

    pub fn width_slices_with_margin(
        self,
        num_items: usize,
        margin: T,
    ) -> impl Iterator<Item = Self> {
        (0..num_items)
            .into_iter()
            .map(move |i| self.width_slice_with_margin(num_items, i, margin))
    }

    pub fn height_slice_with_margin(&self, num_items: usize, index: usize, margin: T) -> Self {
        let num_items = cast::num(num_items);
        let index: T = cast::num(index);
        let total_margin = num_items * margin + margin;
        let items_height = self.height() - total_margin;
        let item_height = items_height / num_items;
        let item_top = self.top + margin + index * (margin + item_height);
        Rect {
            top:    item_top,
            left:   self.left,
            right:  self.right,
            bottom: item_top + item_height,
        }
    }

    pub fn height_slices_with_margin(
        self,
        num_items: usize,
        margin: T,
    ) -> impl Iterator<Item = Self> {
        (0..num_items)
            .into_iter()
            .map(move |i| self.height_slice_with_margin(num_items, i, margin))
    }

    pub fn grid_slices(
        self,
        num_items: Size<usize>,
    ) -> impl Iterator<Item = impl Iterator<Item = Self>> {
        self.grid_slices_with_margin(num_items, T::zero(), T::zero())
    }

    pub fn grid_slices_with_margin(
        self,
        num_items: Size<usize>,
        margin_x: T,
        margin_y: T,
    ) -> impl Iterator<Item = impl Iterator<Item = Self>> {
        self.width_slices_with_margin(num_items.width(), margin_x)
            .map(move |x| x.height_slices_with_margin(num_items.height(), margin_y))
    }

    pub fn grid_cells(self, num_items: Size<usize>) -> impl Iterator<Item = (Point<usize>, Self)> {
        self.grid_cells_with_margin(num_items, T::zero(), T::zero())
    }

    pub fn grid_cells_with_margin(
        self,
        num_items: Size<usize>,
        margin_x: T,
        margin_y: T,
    ) -> impl Iterator<Item = (Point<usize>, Self)> {
        self.grid_slices_with_margin(num_items, margin_x, margin_y)
            .enumerate()
            .flat_map(|(x, column)| {
                column
                    .enumerate()
                    .map(move |(y, cell)| (Point::new(x, y), cell))
            })
    }
}

impl<T: Add<RHS>, RHS: Copy> Add<Vec2<RHS>> for Rect<T> {
    type Output = Rect<T::Output>;
    fn add(self, rhs: Vec2<RHS>) -> Self::Output {
        Rect {
            left:   self.left + rhs.dx,
            top:    self.top + rhs.dy,
            right:  self.right + rhs.dx,
            bottom: self.bottom + rhs.dy,
        }
    }
}

impl<T: AddAssign<RHS>, RHS: Copy> AddAssign<Vec2<RHS>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vec2<RHS>) {
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
