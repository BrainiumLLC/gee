use crate::{
    cast, HorizontalLocation, LineSegment, OrdinaryFloat, OrdinaryNum, Point, RectLocation,
    RectPosition, Size, Vec2, VerticalLocation,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
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
        if cfg!(not(feature = "unchecked-ctors")) {
            Self::try_new(top, right, bottom, left)
                .expect("invalid Rect (left > right and/or top > bottom)")
        } else {
            Self::new(top, right, bottom, left)
        }
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    pub fn with_position(rect_position: RectPosition<T>, size: Size<T>) -> Rect<T> {
        let width = size.width();
        let height = size.height();
        Rect::new(
            rect_position.top_with_height(height),
            rect_position.right_with_width(width),
            rect_position.bottom_with_height(height),
            rect_position.left_with_width(width),
        )
    }

    pub fn with_top_left(top_left: Point<T>, size: Size<T>) -> Self {
        Self::new(
            top_left.y,
            top_left.x + size.width(),
            top_left.y + size.height(),
            top_left.x,
        )
    }

    pub fn with_top_center(top_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        Self::new(
            top_center.y,
            top_center.x + half_width,
            top_center.y + size.height(),
            top_center.x - half_width,
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

    pub fn with_center_left(left_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().halved();
        Self::new(
            left_center.y - half_height,
            left_center.x + size.width(),
            left_center.y + half_height,
            left_center.x,
        )
    }

    pub fn with_center(center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        let half_height = size.height().halved();
        Self::new(
            center.y - half_height,
            center.x + half_width,
            center.y + half_height,
            center.x - half_width,
        )
    }

    pub fn with_center_right(right_center: Point<T>, size: Size<T>) -> Self {
        let half_height = size.height().halved();
        Self::new(
            right_center.y - half_height,
            right_center.x,
            right_center.y + half_height,
            right_center.x - size.width(),
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

    pub fn with_bottom_center(bottom_center: Point<T>, size: Size<T>) -> Self {
        let half_width = size.width().halved();
        Self::new(
            bottom_center.y - size.height(),
            bottom_center.x + half_width,
            bottom_center.y,
            bottom_center.x - half_width,
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

    pub fn size(&self) -> Size<T> {
        Size::new(self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> T
    where
        T: OrdinaryFloat,
    {
        self.size().aspect_ratio()
    }

    pub fn is_empty(&self) -> bool {
        self.top == self.bottom || self.left == self.right
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

    pub fn resize(&self, size: Size<T>, fixed_location: RectLocation) -> Self {
        Self::with_position(self.position_at(fixed_location), size)
    }

    pub fn resize_width(&self, width: T, fixed_location: HorizontalLocation) -> Self {
        self.resize(
            self.size().resize_width(width),
            fixed_location | VerticalLocation::Top,
        )
    }

    pub fn resize_height(&self, height: T, fixed_location: VerticalLocation) -> Self {
        self.resize(
            self.size().resize_height(height),
            HorizontalLocation::Left | fixed_location,
        )
    }

    pub fn resize_uniform(&self, dim: T, fixed_location: RectLocation) -> Self {
        self.resize(Size::square(dim), fixed_location)
    }

    pub fn reposition(&self, position: RectPosition<T>) -> Self {
        Self::with_position(position, self.size())
    }

    pub fn reposition_x(&self, new_x: T, location: HorizontalLocation) -> Self {
        self.reposition(
            self.position_at(location | VerticalLocation::Top)
                .reposition_x(new_x),
        )
    }

    pub fn reposition_y(&self, new_y: T, location: VerticalLocation) -> Self {
        self.reposition(
            self.position_at(HorizontalLocation::Left | location)
                .reposition_y(new_y),
        )
    }

    pub fn scale(&self, scale: Vec2<T>, fixed_location: RectLocation) -> Self {
        self.resize(self.size().scale(scale), fixed_location)
    }

    pub fn scale_width(&self, scale: T, fixed_location: HorizontalLocation) -> Self {
        self.map_width(fixed_location, move |width| width * scale)
    }

    pub fn scale_height(&self, scale: T, fixed_location: VerticalLocation) -> Self {
        self.map_height(fixed_location, move |height| height * scale)
    }

    pub fn scale_uniform(&self, scale: T, fixed_location: RectLocation) -> Self {
        self.resize(self.size().scale_uniform(scale), fixed_location)
    }

    pub fn translate(&self, offset: Vec2<T>) -> Self {
        *self + offset
    }

    pub fn translate_x(&self, offset_x: T) -> Self {
        self.translate(Vec2::from_dx(offset_x))
    }

    pub fn translate_y(&self, offset_y: T) -> Self {
        self.translate(Vec2::from_dy(offset_y))
    }

    pub fn translate_uniform(&self, offset: T) -> Self {
        self.translate(Vec2::uniform(offset))
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

        Self::try_new(top, left, bottom, right)
    }

    pub fn union(&self, other: &Self) -> Self {
        let top = self.top.min(other.top);
        let right = self.right.max(other.right);
        let bottom = self.bottom.max(other.bottom);
        let left = self.left.min(other.left);

        // We are guaranteed a canonical rectangle if both inputs are canonical.
        Self::new_unchecked(top, left, bottom, right)
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
        let num_items: T = cast::num(num_items);
        let index: T = cast::num(index);
        let total_margin = num_items * margin + margin;
        let items_width = self.width() - total_margin;
        let item_width = items_width / num_items;
        let item_left = self.left + margin + index * (margin + item_width);
        Rect::new(self.top, item_left + item_width, self.bottom, item_left)
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
        let num_items: T = cast::num(num_items);
        let index: T = cast::num(index);
        let total_margin = num_items * margin + margin;
        let items_height = self.height() - total_margin;
        let item_height = items_height / num_items;
        let item_top = self.top + margin + index * (margin + item_height);
        Rect::new(item_top, self.right, item_top + item_height, self.left)
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

    pub fn map_size(
        self,
        fixed_location: RectLocation,
        f: impl FnOnce(Size<T>) -> Size<T>,
    ) -> Self {
        self.resize(f(self.size()), fixed_location)
    }

    pub fn map_width(self, fixed_location: HorizontalLocation, f: impl FnOnce(T) -> T) -> Self {
        self.resize_width(f(self.width()), fixed_location)
    }

    pub fn map_height(self, fixed_location: VerticalLocation, f: impl FnOnce(T) -> T) -> Self {
        self.resize_height(f(self.height()), fixed_location)
    }

    pub fn map<U: OrdinaryNum>(self, mut f: impl FnMut(T) -> U) -> Rect<U> {
        Rect::new(f(self.top), f(self.right), f(self.bottom), f(self.left))
    }

    impl_casts_and_cast!(Rect);

    pub fn to_clockwise_array(self) -> [T; 4] {
        [self.top, self.right, self.bottom, self.left]
    }

    pub fn to_clockwise_tuple(self) -> (T, T, T, T) {
        (self.top, self.right, self.bottom, self.left)
    }
}

impl<T: OrdinaryNum> Add<Vec2<T>> for Rect<T> {
    type Output = Self;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Rect::new(
            self.top + rhs.dy,
            self.right + rhs.dx,
            self.bottom + rhs.dy,
            self.left + rhs.dx,
        )
    }
}

impl<T: OrdinaryNum> AddAssign<Vec2<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs
    }
}

impl<T: OrdinaryNum> Sub<Vec2<T>> for Rect<T> {
    type Output = Self;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Rect::new(
            self.top - rhs.dy,
            self.right - rhs.dx,
            self.bottom - rhs.dy,
            self.left - rhs.dx,
        )
    }
}

impl<T: OrdinaryNum> SubAssign<Vec2<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = *self - rhs
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<euclid::Rect<T>> for Rect<T> {
    fn from(rect: euclid::Rect<T>) -> Self {
        Rect::with_top_left(rect.origin.into(), rect.size.into())
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<euclid::Rect<T>> for Rect<T> {
    fn into(self) -> euclid::Rect<T> {
        euclid::Rect::new(self.top_left().into(), self.size().into())
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
        let rect = Rect::new(top, right, bottom, left);

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
        rect_asserts(Rect::with_top_left(
            Point::new(left, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_top_center(
            Point::new(center_x, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_top_right(
            Point::new(right, top),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_center_left(
            Point::new(left, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_center(
            Point::new(center_x, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_center_right(
            Point::new(right, center_y),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_bottom_left(
            Point::new(left, bottom),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_bottom_center(
            Point::new(center_x, bottom),
            Size::new(width, height),
        ));
        rect_asserts(Rect::with_bottom_right(
            Point::new(right, bottom),
            Size::new(width, height),
        ));

        assert_eq!(rect.size(), Size::new(width, height));
    }
}
