use crate::{
    HorizontalLocation, LineSegment, Point, RectLocation, RectPosition, Size, Transform, Vector,
    VerticalLocation,
};
use derive_more::{Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] // TODO: check rect validity in deserialize
#[repr(C)]
pub struct Rect<T = f32> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T: en::Num> Rect<T> {
    pub fn from_top_right_bottom_left(top: T, right: T, bottom: T, left: T) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn zero() -> Self {
        Self::from_top_right_bottom_left(T::zero(), T::zero(), T::zero(), T::zero())
    }

    pub fn from_position(rect_position: RectPosition<T>, size: Size<T>) -> Self {
        let width = size.width();
        let height = size.height();
        Self::from_top_right_bottom_left(
            rect_position.top_with_height(height),
            rect_position.right_with_width(width),
            rect_position.bottom_with_height(height),
            rect_position.left_with_width(width),
        )
    }

    pub fn from_top_left(top_left: Point<T>, size: Size<T>) -> Self {
        Self::from_top_right_bottom_left(
            top_left.y,
            top_left.x + size.width(),
            top_left.y + size.height(),
            top_left.x,
        )
    }

    pub fn from_top_center(top_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        Self::from_top_right_bottom_left(
            top_center.y,
            top_center.x + half_width,
            top_center.y + size.height(),
            top_center.x - half_width,
        )
    }

    pub fn from_top_right(top_right: Point<T>, size: Size<T>) -> Self {
        Self::from_top_right_bottom_left(
            top_right.y,
            top_right.x,
            top_right.y + size.height(),
            top_right.x - size.width(),
        )
    }

    pub fn from_center_left(left_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().halved();
        Self::from_top_right_bottom_left(
            left_center.y - half_height,
            left_center.x + size.width(),
            left_center.y + half_height,
            left_center.x,
        )
    }

    pub fn from_center(center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        let half_height = size.height().halved();
        Self::from_top_right_bottom_left(
            center.y - half_height,
            center.x + half_width,
            center.y + half_height,
            center.x - half_width,
        )
    }

    pub fn from_center_right(right_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().halved();
        Self::from_top_right_bottom_left(
            right_center.y - half_height,
            right_center.x,
            right_center.y + half_height,
            right_center.x - size.width(),
        )
    }

    pub fn from_bottom_right(bottom_right: Point<T>, size: Size<T>) -> Self {
        Self::from_top_right_bottom_left(
            bottom_right.y - size.height(),
            bottom_right.x,
            bottom_right.y,
            bottom_right.x - size.width(),
        )
    }

    pub fn from_bottom_center(bottom_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        Self::from_top_right_bottom_left(
            bottom_center.y - size.height(),
            bottom_center.x + half_width,
            bottom_center.y,
            bottom_center.x - half_width,
        )
    }

    pub fn from_bottom_left(bottom_left: Point<T>, size: Size<T>) -> Self {
        Self::from_top_right_bottom_left(
            bottom_left.y - size.height(),
            bottom_left.x + size.width(),
            bottom_left.y,
            bottom_left.x,
        )
    }

    pub fn from_iter<I>(points: I) -> Self
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
        Self::from_top_right_bottom_left(min_y, max_x, max_y, min_x)
    }

    pub fn from_points(a: Point<T>, b: Point<T>) -> Self {
        Self::from_top_right_bottom_left(a.y.min(b.y), a.x.max(b.x), a.y.max(b.y), a.x.min(b.x))
    }

    pub fn top(&self) -> T {
        self.top
    }

    pub fn right(&self) -> T {
        self.right
    }

    pub fn bottom(&self) -> T {
        self.bottom
    }

    pub fn left(&self) -> T {
        self.left
    }

    pub fn center_x(&self) -> T {
        (self.left + self.right).halved()
    }

    pub fn center_y(&self) -> T {
        (self.top + self.bottom).halved()
    }

    pub fn width(&self) -> T {
        self.right - self.left
    }

    pub fn height(&self) -> T {
        self.bottom - self.top
    }

    pub fn top_left(&self) -> Point<T> {
        Point::new(self.left, self.top)
    }

    pub fn top_center(&self) -> Point<T> {
        Point::new(self.center_x(), self.top)
    }

    pub fn top_right(&self) -> Point<T> {
        Point::new(self.right, self.top)
    }

    pub fn center_left(&self) -> Point<T> {
        Point::new(self.left, self.center_y())
    }

    pub fn center(&self) -> Point<T> {
        Point::new(self.center_x(), self.center_y())
    }

    pub fn center_right(&self) -> Point<T> {
        Point::new(self.right, self.center_y())
    }

    pub fn bottom_left(&self) -> Point<T> {
        Point::new(self.left, self.bottom)
    }

    pub fn bottom_center(&self) -> Point<T> {
        Point::new(self.center_x(), self.bottom)
    }

    pub fn bottom_right(&self) -> Point<T> {
        Point::new(self.right, self.bottom)
    }

    pub fn clockwise_points(&self) -> impl Iterator<Item = Point<T>> + Clone + DoubleEndedIterator {
        std::iter::once(self.top_left())
            .chain(std::iter::once(self.top_right()))
            .chain(std::iter::once(self.bottom_right()))
            .chain(std::iter::once(self.bottom_left()))
    }

    pub fn to_clockwise_array(&self) -> [T; 4] {
        [self.top, self.right, self.bottom, self.left]
    }

    pub fn to_clockwise_tuple(&self) -> (T, T, T, T) {
        (self.top, self.right, self.bottom, self.left)
    }

    pub fn size(&self) -> Size<T> {
        Size::new(self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> T
    where
        T: en::Float,
    {
        self.size().aspect_ratio()
    }

    /// Returns `true` if the rect's area is greater than 0.
    pub fn has_area(&self) -> bool {
        self.size().min_dim() > T::zero()
    }

    pub fn contains_x(&self, x: T) -> bool {
        (self.left..self.right).contains(&x)
    }

    pub fn contains_y(&self, y: T) -> bool {
        (self.top..self.bottom).contains(&y)
    }

    pub fn contains(&self, point: Point<T>) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn contains_inclusive_x(&self, x: T) -> bool {
        (self.left..=self.right).contains(&x)
    }

    pub fn contains_inclusive_y(&self, y: T) -> bool {
        (self.top..=self.bottom).contains(&y)
    }

    pub fn contains_inclusive(&self, point: Point<T>) -> bool {
        self.contains_inclusive_x(point.x) && self.contains_inclusive_y(point.y)
    }

    pub fn grow_to(&self, point: Point<T>) -> Self {
        if self.contains_inclusive(point) {
            *self
        } else {
            let a = std::iter::once(self.bottom_right());
            let b = std::iter::once(self.top_left());
            let c = std::iter::once(point);
            let rect = Self::from_iter(a.chain(b).chain(c));
            debug_assert!(rect.contains_inclusive(point));
            rect
        }
    }

    pub fn point_at(&self, location: RectLocation) -> Point<T> {
        let x = match location.horizontal {
            HorizontalLocation::Left => self.left(),
            HorizontalLocation::Center => self.center_x(),
            HorizontalLocation::Right => self.right(),
        };
        let y = match location.vertical {
            VerticalLocation::Top => self.top(),
            VerticalLocation::Center => self.center_y(),
            VerticalLocation::Bottom => self.bottom(),
        };
        Point::new(x, y)
    }

    pub fn position_at(&self, location: RectLocation) -> RectPosition<T> {
        RectPosition::new(location, self.point_at(location))
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
                Self::from_top_right_bottom_left(top, x, bottom, left),
                Self::from_top_right_bottom_left(top, right, bottom, x),
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
                Self::from_top_right_bottom_left(top, right, y, left),
                Self::from_top_right_bottom_left(y, right, bottom, left),
            ))
        } else {
            None
        }
    }

    pub fn split_at_ratio_width(&self, ratio: T) -> Option<(Self, Self)>
    where
        T: en::Float,
    {
        self.split_at_x(self.width() * ratio)
    }

    pub fn split_at_ratio_height(&self, ratio: T) -> Option<(Self, Self)>
    where
        T: en::Float,
    {
        self.split_at_y(self.height() * ratio)
    }

    pub fn with_position(&self, position: RectPosition<T>) -> Self {
        Self::from_position(position, self.size())
    }

    pub fn with_x(&self, new_x: T, location: HorizontalLocation) -> Self {
        self.with_position(
            self.position_at(location | VerticalLocation::Top)
                .with_x(new_x),
        )
    }

    pub fn with_y(&self, new_y: T, location: VerticalLocation) -> Self {
        self.with_position(
            self.position_at(HorizontalLocation::Left | location)
                .with_y(new_y),
        )
    }

    pub fn with_size(&self, size: Size<T>, fixed_location: RectLocation) -> Self {
        Self::from_position(self.position_at(fixed_location), size)
    }

    pub fn with_width(&self, width: T, fixed_location: HorizontalLocation) -> Self {
        self.with_size(
            self.size().with_width(width),
            fixed_location | VerticalLocation::Top,
        )
    }

    pub fn with_height(&self, height: T, fixed_location: VerticalLocation) -> Self {
        self.with_size(
            self.size().with_height(height),
            HorizontalLocation::Left | fixed_location,
        )
    }

    pub fn scale(&self, scale: Vector<T>, fixed_location: RectLocation) -> Self {
        self.with_size(self.size().scale(scale), fixed_location)
    }

    pub fn scale_width(&self, scale: T, fixed_location: HorizontalLocation) -> Self {
        self.map_width(move |width| width * scale, fixed_location)
    }

    pub fn scale_height(&self, scale: T, fixed_location: VerticalLocation) -> Self {
        self.map_height(move |height| height * scale, fixed_location)
    }

    pub fn scale_uniform(&self, scale: T, fixed_location: RectLocation) -> Self {
        self.with_size(self.size().scale_uniform(scale), fixed_location)
    }

    pub fn translate(&self, offset: Vector<T>) -> Self {
        *self + offset
    }

    pub fn translate_x(&self, offset_x: T) -> Self {
        self.translate(Vector::from_dx(offset_x))
    }

    pub fn translate_y(&self, offset_y: T) -> Self {
        self.translate(Vector::from_dy(offset_y))
    }

    // Inspired by https://api.flutter.dev/flutter/painting/EdgeInsets-class.html
    pub fn inset(&self, top: T, right: T, bottom: T, left: T) -> Self {
        Self::from_top_right_bottom_left(
            self.top + top,
            self.right - right,
            self.bottom - bottom,
            self.left + left,
        )
    }

    pub fn inset_symmetric(&self, horiz: T, vert: T) -> Self {
        self.inset(vert, horiz, vert, horiz)
    }

    pub fn inset_uniform(&self, inset: T) -> Self {
        self.inset_symmetric(inset, inset)
    }

    pub fn inset_top(&self, top: T) -> Self {
        self.inset(top, T::zero(), T::zero(), T::zero())
    }

    pub fn inset_right(&self, right: T) -> Self {
        self.inset(T::zero(), right, T::zero(), T::zero())
    }

    pub fn inset_bottom(&self, bottom: T) -> Self {
        self.inset(T::zero(), T::zero(), bottom, T::zero())
    }

    pub fn inset_left(&self, left: T) -> Self {
        self.inset(T::zero(), T::zero(), T::zero(), left)
    }

    pub fn outset(&self, top: T, right: T, bottom: T, left: T) -> Self {
        Self::from_top_right_bottom_left(
            self.top - top,
            self.right + right,
            self.bottom + bottom,
            self.left - left,
        )
    }

    pub fn outset_symmetric(&self, horiz: T, vert: T) -> Self {
        self.outset(vert, horiz, vert, horiz)
    }

    pub fn outset_uniform(&self, outset: T) -> Self {
        self.outset_symmetric(outset, outset)
    }

    pub fn outset_top(&self, top: T) -> Self {
        self.outset(top, T::zero(), T::zero(), T::zero())
    }

    pub fn outset_right(&self, right: T) -> Self {
        self.outset(T::zero(), right, T::zero(), T::zero())
    }

    pub fn outset_bottom(&self, bottom: T) -> Self {
        self.outset(T::zero(), T::zero(), bottom, T::zero())
    }

    pub fn outset_left(&self, left: T) -> Self {
        self.outset(T::zero(), T::zero(), T::zero(), left)
    }

    pub fn transform(self, transform: Transform<T>) -> Self {
        Self::from_iter(
            self.clockwise_points()
                .map(|point| point.transform(transform)),
        )
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

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let top = self.top.max(other.top);
        let right = self.right.min(other.right);
        let bottom = self.bottom.min(other.bottom);
        let left = self.left.max(other.left);
        Some(Self::from_top_right_bottom_left(top, right, bottom, left)).filter(Self::has_area)
    }

    pub fn union(&self, other: &Self) -> Self {
        let top = self.top.min(other.top);
        let right = self.right.max(other.right);
        let bottom = self.bottom.max(other.bottom);
        let left = self.left.min(other.left);
        Self::from_top_right_bottom_left(top, right, bottom, left)
    }

    pub fn width_slice(&self, num_items: usize, index: usize) -> Self {
        self.width_slice_with_margin(num_items, index, T::zero())
    }

    pub fn width_slices(&self, num_items: usize) -> impl Iterator<Item = Self> {
        self.width_slices_with_margin(num_items, T::zero())
    }

    pub fn height_slice(&self, num_items: usize, index: usize) -> Self {
        self.height_slice_with_margin(num_items, index, T::zero())
    }

    pub fn height_slices(&self, num_items: usize) -> impl Iterator<Item = Self> {
        self.height_slices_with_margin(num_items, T::zero())
    }

    pub fn width_slice_with_margin(&self, num_items: usize, index: usize, margin: T) -> Self {
        let num_items: T = en::cast(num_items);
        let index: T = en::cast(index);
        let total_margin = num_items * margin + margin;
        let items_width = self.width() - total_margin;
        let item_width = items_width / num_items;
        let item_left = self.left + margin + index * (margin + item_width);
        Self::from_top_right_bottom_left(self.top, item_left + item_width, self.bottom, item_left)
    }

    pub fn width_slices_with_margin(
        &self,
        num_items: usize,
        margin: T,
    ) -> impl Iterator<Item = Self> {
        let this = *self;
        (0..num_items)
            .into_iter()
            .map(move |i| this.width_slice_with_margin(num_items, i, margin))
    }

    pub fn height_slice_with_margin(&self, num_items: usize, index: usize, margin: T) -> Self {
        let num_items: T = en::cast(num_items);
        let index: T = en::cast(index);
        let total_margin = num_items * margin + margin;
        let items_height = self.height() - total_margin;
        let item_height = items_height / num_items;
        let item_top = self.top + margin + index * (margin + item_height);
        Self::from_top_right_bottom_left(item_top, self.right, item_top + item_height, self.left)
    }

    pub fn height_slices_with_margin(
        &self,
        num_items: usize,
        margin: T,
    ) -> impl Iterator<Item = Self> {
        let this = *self;
        (0..num_items)
            .into_iter()
            .map(move |i| this.height_slice_with_margin(num_items, i, margin))
    }

    pub fn grid_slices(
        &self,
        num_items: Size<usize>,
    ) -> impl Iterator<Item = impl Iterator<Item = Self>> {
        self.grid_slices_with_margin(num_items, T::zero(), T::zero())
    }

    pub fn grid_slices_with_margin(
        &self,
        num_items: Size<usize>,
        margin_x: T,
        margin_y: T,
    ) -> impl Iterator<Item = impl Iterator<Item = Self>> {
        self.height_slices_with_margin(num_items.height(), margin_y)
            .map(move |x| x.width_slices_with_margin(num_items.width(), margin_x))
    }

    pub fn grid_cells(&self, num_items: Size<usize>) -> impl Iterator<Item = (Point<usize>, Self)> {
        self.grid_cells_with_margin(num_items, T::zero(), T::zero())
    }

    pub fn grid_cells_with_margin(
        &self,
        num_items: Size<usize>,
        margin_x: T,
        margin_y: T,
    ) -> impl Iterator<Item = (Point<usize>, Self)> {
        self.grid_slices_with_margin(num_items, margin_x, margin_y)
            .enumerate()
            .flat_map(|(y, row)| {
                row.enumerate()
                    .map(move |(x, cell)| (Point::new(x, y), cell))
            })
    }

    pub fn map<U: en::Num>(self, mut f: impl FnMut(T) -> U) -> Rect<U> {
        Rect::from_top_right_bottom_left(f(self.top), f(self.right), f(self.bottom), f(self.left))
    }

    pub fn map_size(
        self,
        f: impl FnOnce(Size<T>) -> Size<T>,
        fixed_location: RectLocation,
    ) -> Self {
        self.with_size(f(self.size()), fixed_location)
    }

    pub fn map_width(self, f: impl FnOnce(T) -> T, fixed_location: HorizontalLocation) -> Self {
        self.with_width(f(self.width()), fixed_location)
    }

    pub fn map_height(self, f: impl FnOnce(T) -> T, fixed_location: VerticalLocation) -> Self {
        self.with_height(f(self.height()), fixed_location)
    }

    impl_casts_and_cast!(Rect);

    #[cfg(feature = "d6")]
    pub fn random_point(&self) -> Point<T>
    where
        T: d6::rand::distributions::uniform::SampleUniform,
    {
        let point = Point::new(
            d6::range(self.left()..=self.right()),
            d6::range(self.top()..=self.bottom()),
        );
        debug_assert!(self.contains_inclusive(point));
        point
    }
}

impl<T: en::Num> Add<Vector<T>> for Rect<T> {
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Rect::from_top_right_bottom_left(
            self.top + rhs.dy,
            self.right + rhs.dx,
            self.bottom + rhs.dy,
            self.left + rhs.dx,
        )
    }
}

impl<T: en::Num> AddAssign<Vector<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub<Vector<T>> for Rect<T> {
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Rect::from_top_right_bottom_left(
            self.top - rhs.dy,
            self.right - rhs.dx,
            self.bottom - rhs.dy,
            self.left - rhs.dx,
        )
    }
}

impl<T: en::Num> SubAssign<Vector<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs
    }
}

impl<T: en::Num> Mul<Vector<T>> for Rect<T> {
    type Output = Self;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Rect::from_top_right_bottom_left(
            self.top * rhs.dy,
            self.right * rhs.dx,
            self.bottom * rhs.dy,
            self.left * rhs.dx,
        )
    }
}

impl<T: en::Num> MulAssign<Vector<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: Vector<T>) {
        *self = *self * rhs
    }
}

impl<T: en::Num> Div<Vector<T>> for Rect<T> {
    type Output = Self;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Rect::from_top_right_bottom_left(
            self.top / rhs.dy,
            self.right / rhs.dx,
            self.bottom / rhs.dy,
            self.left / rhs.dx,
        )
    }
}

impl<T: en::Num> DivAssign<Vector<T>> for Rect<T> {
    fn div_assign(&mut self, rhs: Vector<T>) {
        *self = *self / rhs
    }
}

impl<T: en::Num> Rem<Vector<T>> for Rect<T> {
    type Output = Self;

    fn rem(self, rhs: Vector<T>) -> Self::Output {
        Rect::from_top_right_bottom_left(
            self.top % rhs.dy,
            self.right % rhs.dx,
            self.bottom % rhs.dy,
            self.left % rhs.dx,
        )
    }
}

impl<T: en::Num> RemAssign<Vector<T>> for Rect<T> {
    fn rem_assign(&mut self, rhs: Vector<T>) {
        *self = *self % rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num, U> From<Rect<T>> for euclid::Rect<T, U> {
    fn from(r: Rect<T>) -> euclid::Rect<T, U> {
        Self::new(r.top_left().into(), r.size().into())
    }
}

#[cfg(feature = "euclid")]
impl<T: en::Num, U> From<euclid::Rect<T, U>> for Rect<T> {
    fn from(r: euclid::Rect<T, U>) -> Rect<T> {
        Self::from_top_left(r.origin.into(), r.size.into())
    }
}

/// An axis-aligned bounding box. Identical to `Rect`, but newtyped to flag it as a different concept.
/// The `Rect` API is accessible through `Deref`/`DerefMut`.
#[derive(
    Clone, Copy, Debug, Default, Deref, DerefMut, From, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AABB<T>(Rect<T>);

impl<T: en::Num> From<AABB<T>> for Rect<T> {
    fn from(a: AABB<T>) -> Self {
        *a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn points() {
        let top = -1;
        let right = 4;
        let bottom = 7;
        let left = -2;
        let center_x = 1;
        let center_y = 3;
        let width = 6;
        let height = 8;
        let rect = Rect::from_top_right_bottom_left(top, right, bottom, left);

        let rect_asserts = |rect: Rect<i32>| {
            assert_eq!(rect.top(), top);
            assert_eq!(rect.right(), right);
            assert_eq!(rect.bottom(), bottom);
            assert_eq!(rect.left(), left);
            assert_eq!(rect.width(), width);
            assert_eq!(rect.height(), height);

            assert_eq!(rect.top_left(), Point::new(left, top));
            assert_eq!(rect.top_center(), Point::new(center_x, top));
            assert_eq!(rect.top_right(), Point::new(right, top));
            assert_eq!(rect.center_left(), Point::new(left, center_y));
            assert_eq!(rect.center(), Point::new(center_x, center_y));
            assert_eq!(rect.center_right(), Point::new(right, center_y));
            assert_eq!(rect.bottom_left(), Point::new(left, bottom));
            assert_eq!(rect.bottom_center(), Point::new(center_x, bottom));
            assert_eq!(rect.bottom_right(), Point::new(right, bottom));
        };
        rect_asserts(rect);
        rect_asserts(Rect::from_top_left(
            Point::new(left, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_top_center(
            Point::new(center_x, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_top_right(
            Point::new(right, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_center_left(
            Point::new(left, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_center(
            Point::new(center_x, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_center_right(
            Point::new(right, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_bottom_left(
            Point::new(left, bottom),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_bottom_center(
            Point::new(center_x, bottom),
            Size::new(width, height),
        ));
        rect_asserts(Rect::from_bottom_right(
            Point::new(right, bottom),
            Size::new(width, height),
        ));

        assert_eq!(rect.size(), Size::new(width, height));
    }

    #[test]
    fn has_area() {
        let normal = Rect::from_top_left(Point::zero(), Size::square(100));
        assert!(
            normal.has_area(),
            "erroneously found `{:?}` to have no area",
            normal
        );
        let empty = Rect::from_top_left(Point::new(50, 50), Size::zero());
        assert!(
            !empty.has_area(),
            "erroneously found `{:?}` to have an area",
            empty
        );
        let inverted = Rect::from_top_left(Point::zero(), Size::square(-100));
        assert!(
            !inverted.has_area(),
            "erroneously found `{:?}` to have an area",
            inverted
        );
    }

    #[test]
    fn intersection() {
        let offset = Point::new(50, 50);
        let size = Size::square(100);
        let a = Rect::from_top_left(Point::zero(), size);
        let b = Rect::from_top_left(offset, size);
        let c = Rect::from_top_left(offset, size - offset.to_vector());
        assert_eq!(
            a.intersection(&b),
            Some(c),
            "erroneously found no intersection between `{:?}` and `{:?}`",
            a,
            b
        );
    }

    #[test]
    fn grow_to() {
        let rect = Rect::from_top_left(Point::new(10, 10), Size::new(10, 10));
        assert_eq!(
            rect.grow_to(Point::new(0, 20)),
            Rect::from_top_left(Point::new(0, 10), Size::new(20, 10))
        );
        assert_eq!(
            rect.grow_to(Point::new(20, 0)),
            Rect::from_top_left(Point::new(10, 0), Size::new(10, 20))
        );
    }
}
