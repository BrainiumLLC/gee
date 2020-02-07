use crate::{
    cast, lerp_half, line_segment::LineSegment, max::Max, min::Min, point::Point, size::Size,
    vec2::Vec2,
};
use num_traits::{Num, NumCast, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Rect<T> {
    pub left:   T,
    pub top:    T,
    pub right:  T,
    pub bottom: T,
}

impl<T: Add<Output = T> + Copy> Rect<T> {
    pub fn with_top_left(top_left: Point<T>, size: Size<T>) -> Self {
        Self {
            left:   top_left.x,
            top:    top_left.y,
            right:  top_left.x + size.width,
            bottom: top_left.y + size.height,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Rect<T> {
    pub fn with_top_right(top_right: Point<T>, size: Size<T>) -> Self {
        Self {
            left:   top_right.x - size.width,
            top:    top_right.y,
            right:  top_right.x,
            bottom: top_right.y + size.height,
        }
    }

    pub fn with_bottom_right(bottom_right: Point<T>, size: Size<T>) -> Self {
        Self {
            left:   bottom_right.x - size.width,
            top:    bottom_right.y - size.height,
            right:  bottom_right.x,
            bottom: bottom_right.y,
        }
    }

    pub fn with_bottom_left(bottom_left: Point<T>, size: Size<T>) -> Self {
        Self {
            left:   bottom_left.x,
            top:    bottom_left.y - size.height,
            right:  bottom_left.x + size.width,
            bottom: bottom_left.y,
        }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + From<u8> + Default + Sub<Output = T> + Div<Output = T>,
{
    pub fn with_center(center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width / 2.into();
        let half_height = size.height / 2.into();
        Self {
            left:   center.x - half_width,
            top:    center.y - half_height,
            right:  center.x + half_width,
            bottom: center.y + half_height,
        }
    }

    pub fn with_top_center(top_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width / 2.into();
        Self {
            left:   top_center.x - half_width,
            top:    top_center.y,
            right:  top_center.x + half_width,
            bottom: top_center.y + size.height,
        }
    }

    pub fn with_bottom_center(bottom_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width / 2.into();
        Self {
            left:   bottom_center.x - half_width,
            top:    bottom_center.y - size.height,
            right:  bottom_center.x + half_width,
            bottom: bottom_center.y,
        }
    }

    pub fn with_left_center(left_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height / 2.into();
        Self {
            left:   left_center.x,
            top:    left_center.y - half_height,
            right:  left_center.x + size.width,
            bottom: left_center.y + half_height,
        }
    }

    pub fn with_right_center(right_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height / 2.into();
        Self {
            left:   right_center.x - size.width,
            top:    right_center.y - half_height,
            right:  right_center.x,
            bottom: right_center.y + half_height,
        }
    }
}

impl<T: Copy + PartialOrd> Rect<T> {
    pub fn split_at_x(&self, x: T) -> Option<(Self, Self)> {
        if self.contains_x(x) {
            let Self { left, right, top, bottom } = *self;
            Some((
                Self {
                    left,
                    right: x,
                    top,
                    bottom, 
                },
                Self {
                    left: x,
                    right,
                    top,
                    bottom,
                },
            ))
        } else {
            None
        }
    }

    pub fn split_at_y(&self, y: T) -> Option<(Self, Self)> {
        if self.contains_y(y) {
            let Self { left, right, top, bottom } = *self;
            Some((
                Self {
                    left,
                    right,
                    top,
                    bottom: y, 
                },
                Self {
                    left,
                    right,
                    top: y,
                    bottom,
                },
            ))
        } else {
            None
        }
    }
}

impl<T: Copy + Debug + Num + PartialOrd> Rect<T> {
    pub fn padded(&self, left: T, right: T, top: T, bottom: T) -> Self {
        Self::try_new_checked(
            self.left + left,
            self.right - right,
            self.top + top,
            self.bottom - bottom,
        )
    }

    pub fn padded_horiz_and_vert(&self, horiz: T, vert: T) -> Self {
        self.padded(horiz, horiz, vert, vert)
    }

    pub fn padded_uniform(&self, pad: T) -> Self {
        self.padded_horiz_and_vert(pad, pad)
    }
}

impl<T: Copy> Rect<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    pub fn scaled_from_top(&self, scale: T) -> Self {
        Self {
            left:   self.left,
            top:    self.top,
            right:  self.right,
            bottom: self.top + self.height() * scale,
        }
    }

    pub fn scaled_from_bottom(&self, scale: T) -> Self {
        Self {
            left:   self.left,
            top:    self.bottom - self.height() * scale,
            right:  self.right,
            bottom: self.bottom,
        }
    }

    pub fn scaled_from_left(&self, scale: T) -> Self {
        Self {
            left:   self.left,
            top:    self.top,
            right:  self.left + self.width() * scale,
            bottom: self.bottom,
        }
    }

    pub fn scaled_from_right(&self, scale: T) -> Self {
        Self {
            left:   self.right - self.width() * scale,
            top:    self.top,
            right:  self.right,
            bottom: self.bottom,
        }
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
    pub fn width(&self) -> T::Output {
        self.right - self.left
    }
    pub fn height(&self) -> T::Output {
        self.bottom - self.top
    }
    pub fn size(&self) -> Size<T::Output> {
        Size::new(self.width(), self.height())
    }
}

impl<T, V> Rect<T>
where
    T: Copy + Sub<Output = V>,
    V: Div,
{
    pub fn aspect_ratio(&self) -> V::Output {
        self.size().aspect_ratio()
    }
}

impl<T: Copy + PartialOrd> Rect<T> {
    pub fn contains_x(&self, x: T) -> bool {
        (self.left..self.right).contains(&x)
    }

    pub fn contains_y(&self, y: T) -> bool {
        (self.top..self.bottom).contains(&y)
    }

    pub fn contains(&self, point: &Point<T>) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn valid(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    pub fn new_checked(left: T, right: T, top: T, bottom: T) -> Option<Self> {
        let this = Self { left, right, top, bottom };
        if this.valid() {
            Some(this)
        } else {
            None
        }
    }
}

impl<T: Copy + Debug + PartialOrd> Rect<T> {
    pub fn try_new_checked(left: T, right: T, top: T, bottom: T) -> Self {
        let this = Self { left, right, top, bottom };
        assert!(this.valid(), "invalid rect: {:#?}", this);
        this
    }
}

impl<T: Zero> Rect<T> {
    pub fn zero() -> Self {
        Self {
            left:   T::zero(),
            top:    T::zero(),
            right:  T::zero(),
            bottom: T::zero(),
        }
    }
}

impl<T> Rect<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Rect<U> {
        Rect {
            left:   f(self.left),
            top:    f(self.top),
            right:  f(self.right),
            bottom: f(self.bottom),
        }
    }
}

impl<T: PartialEq> Rect<T> {
    pub fn is_empty(&self) -> bool {
        self.top == self.bottom || self.left == self.right
    }
}

impl<T: Copy> Rect<T> {
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
        Self {
            left:   min_x,
            top:    min_y,
            right:  max_x,
            bottom: max_y,
        }
    }
}

impl<T: Copy + Max + Min> Rect<T> {
    pub fn from_points(a: Point<T>, b: Point<T>) -> Self {
        Self {
            left:   a.x.min(b.x),
            top:    a.y.min(b.y),
            right:  a.x.max(b.x),
            bottom: a.y.max(b.y),
        }
    }
}

impl<T: Copy + Add<Output = U>, U: Div + From<u8>> Rect<T> {
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

impl<T: Copy + Add<Output = U>, U: Div<Output = T> + From<u8>> Rect<T> {
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

impl<T: Copy + PartialOrd + Max + Min> Rect<T> {
    // TODO: clamp method
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

impl<T: Copy + Num + NumCast> Rect<T> {
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

    pub fn width_slices_with_margin(self, num_items: usize, margin: T) -> impl Iterator<Item = Self> {
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

    pub fn height_slices_with_margin(self, num_items: usize, margin: T) -> impl Iterator<Item = Self> {
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
        self.width_slices_with_margin(num_items.width, margin_x)
            .map(move |x| x.height_slices_with_margin(num_items.height, margin_y))
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
