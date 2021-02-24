use crate::{Point, Rect, Size, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, BitOr, Neg, Sub, SubAssign};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HorizontalLocation {
    Left,
    Center,
    Right,
}

impl Neg for HorizontalLocation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use HorizontalLocation::*;
        match self {
            Left => Right,
            Center => Center,
            Right => Left,
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerticalLocation {
    Top,
    Center,
    Bottom,
}

impl BitOr<HorizontalLocation> for VerticalLocation {
    type Output = RectLocation;

    fn bitor(self, rhs: HorizontalLocation) -> Self::Output {
        RectLocation::new(rhs, self)
    }
}

impl BitOr<VerticalLocation> for HorizontalLocation {
    type Output = RectLocation;

    fn bitor(self, rhs: VerticalLocation) -> Self::Output {
        rhs | self
    }
}

impl Neg for VerticalLocation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use VerticalLocation::*;
        match self {
            Top => Bottom,
            Center => Center,
            Bottom => Top,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RectLocation {
    pub horizontal: HorizontalLocation,
    pub vertical: VerticalLocation,
}

impl Neg for RectLocation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.horizontal, -self.vertical)
    }
}

impl RectLocation {
    pub fn new(horizontal: HorizontalLocation, vertical: VerticalLocation) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn left(vertical: VerticalLocation) -> Self {
        HorizontalLocation::Left | vertical
    }

    pub fn right(vertical: VerticalLocation) -> Self {
        HorizontalLocation::Right | vertical
    }

    pub fn top(horizontal: HorizontalLocation) -> Self {
        horizontal | VerticalLocation::Top
    }

    pub fn bottom(horizontal: HorizontalLocation) -> Self {
        horizontal | VerticalLocation::Bottom
    }

    pub fn top_left() -> Self {
        HorizontalLocation::Left | VerticalLocation::Top
    }

    pub fn top_center() -> Self {
        HorizontalLocation::Center | VerticalLocation::Top
    }

    pub fn top_right() -> Self {
        HorizontalLocation::Right | VerticalLocation::Top
    }

    pub fn center_left() -> Self {
        HorizontalLocation::Left | VerticalLocation::Center
    }

    pub fn center() -> Self {
        HorizontalLocation::Center | VerticalLocation::Center
    }

    pub fn center_right() -> Self {
        HorizontalLocation::Right | VerticalLocation::Center
    }

    pub fn bottom_left() -> Self {
        HorizontalLocation::Left | VerticalLocation::Bottom
    }

    pub fn bottom_center() -> Self {
        HorizontalLocation::Center | VerticalLocation::Bottom
    }

    pub fn bottom_right() -> Self {
        HorizontalLocation::Right | VerticalLocation::Bottom
    }

    pub fn relocate_horizontal(self, horizontal: HorizontalLocation) -> Self {
        Self::new(horizontal, self.vertical)
    }

    pub fn relocate_vertical(self, vertical: VerticalLocation) -> Self {
        Self::new(self.horizontal, vertical)
    }

    pub fn point_from_rect<T: en::Num>(self, rect: Rect<T>) -> Point<T> {
        rect.point_at(self)
    }

    pub fn position_from_rect<T: en::Num>(self, rect: Rect<T>) -> RectPosition<T> {
        rect.position_at(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RectPosition<T> {
    pub location: RectLocation,
    pub point: Point<T>,
}

impl<T: en::Num> RectPosition<T> {
    pub fn new(location: RectLocation, point: Point<T>) -> Self {
        Self { location, point }
    }

    pub fn from_location_and_rect(location: RectLocation, rect: Rect<T>) -> Self {
        location.position_from_rect(rect)
    }

    pub fn top_left(point: Point<T>) -> Self {
        Self::new(RectLocation::top_left(), point)
    }

    pub fn top_center(point: Point<T>) -> Self {
        Self::new(RectLocation::top_center(), point)
    }

    pub fn top_right(point: Point<T>) -> Self {
        Self::new(RectLocation::top_right(), point)
    }

    pub fn center_left(point: Point<T>) -> Self {
        Self::new(RectLocation::center_left(), point)
    }

    pub fn center(point: Point<T>) -> Self {
        Self::new(RectLocation::center(), point)
    }

    pub fn center_right(point: Point<T>) -> Self {
        Self::new(RectLocation::center_right(), point)
    }

    pub fn bottom_left(point: Point<T>) -> Self {
        Self::new(RectLocation::bottom_left(), point)
    }

    pub fn bottom_center(point: Point<T>) -> Self {
        Self::new(RectLocation::bottom_center(), point)
    }

    pub fn bottom_right(point: Point<T>) -> Self {
        Self::new(RectLocation::bottom_right(), point)
    }

    pub fn top_left_from_rect(rect: Rect<T>) -> Self {
        Self::top_left(rect.top_left())
    }

    pub fn top_center_from_rect(rect: Rect<T>) -> Self {
        Self::top_center(rect.top_center())
    }

    pub fn top_right_from_rect(rect: Rect<T>) -> Self {
        Self::top_right(rect.top_right())
    }

    pub fn center_left_from_rect(rect: Rect<T>) -> Self {
        Self::center_left(rect.center_left())
    }

    pub fn center_from_rect(rect: Rect<T>) -> Self {
        Self::center(rect.center())
    }

    pub fn center_right_from_rect(rect: Rect<T>) -> Self {
        Self::center_right(rect.center_right())
    }

    pub fn bottom_left_from_rect(rect: Rect<T>) -> Self {
        Self::bottom_left(rect.bottom_left())
    }

    pub fn bottom_center_from_rect(rect: Rect<T>) -> Self {
        Self::bottom_center(rect.bottom_center())
    }

    pub fn bottom_right_from_rect(rect: Rect<T>) -> Self {
        Self::bottom_right(rect.bottom_right())
    }

    pub fn with_location(self, location: RectLocation) -> Self {
        Self::new(location, self.point)
    }

    pub fn with_horizontal(self, horizontal: HorizontalLocation) -> Self {
        self.with_location(self.location.relocate_horizontal(horizontal))
    }

    pub fn with_vertical(self, vertical: VerticalLocation) -> Self {
        self.with_location(self.location.relocate_vertical(vertical))
    }

    pub fn with_point(self, point: Point<T>) -> Self {
        Self::new(self.location, point)
    }

    pub fn with_x(self, x: T) -> Self {
        self.with_point(self.point.with_x(x))
    }

    pub fn with_y(self, y: T) -> Self {
        self.with_point(self.point.with_y(y))
    }

    pub(crate) fn left_with_width(self, width: T) -> T {
        use HorizontalLocation::*;
        match self.location.horizontal {
            Left => self.point.x,
            Center => self.point.x - width.halved(),
            Right => self.point.x - width,
        }
    }

    pub(crate) fn right_with_width(self, width: T) -> T {
        use HorizontalLocation::*;
        match self.location.horizontal {
            Left => self.point.x + width,
            Center => self.point.x + width.halved(),
            Right => self.point.x,
        }
    }

    pub(crate) fn top_with_height(self, height: T) -> T {
        use VerticalLocation::*;
        match self.location.vertical {
            Top => self.point.y,
            Center => self.point.y + height.halved(),
            Bottom => self.point.y + height,
        }
    }

    pub(crate) fn bottom_with_height(self, height: T) -> T {
        use VerticalLocation::*;
        match self.location.vertical {
            Top => self.point.y - height,
            Center => self.point.y - height.halved(),
            Bottom => self.point.y,
        }
    }

    pub fn to_rect(self, size: Size<T>) -> Rect<T> {
        Rect::from_position(self, size)
    }
}

impl<T: en::Num> Add<Vector<T>> for RectPosition<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        RectPosition {
            location: self.location,
            point: self.point + rhs,
        }
    }
}

impl<T: en::Num> AddAssign<Vector<T>> for RectPosition<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub<Vector<T>> for RectPosition<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        RectPosition {
            location: self.location,
            point: self.point - rhs,
        }
    }
}

impl<T: en::Num> SubAssign<Vector<T>> for RectPosition<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horizontal_location_neg() {
        let left = HorizontalLocation::Left;
        let center = HorizontalLocation::Center;
        let right = HorizontalLocation::Right;
        assert_eq!(left, -right);
        assert_eq!(right, -left);
        assert_eq!(center, -center);
    }
    #[test]
    fn vertical_location_neg() {
        let top = VerticalLocation::Top;
        let center = VerticalLocation::Center;
        let bottom = VerticalLocation::Bottom;
        assert_eq!(top, -bottom);
        assert_eq!(bottom, -top);
        assert_eq!(center, -center);
    }

    #[test]
    fn rect_location_neg() {
        assert_eq!(RectLocation::top_left(), -RectLocation::bottom_right());
        assert_eq!(RectLocation::top_center(), -RectLocation::bottom_center());
        assert_eq!(RectLocation::top_right(), -RectLocation::bottom_left());
        assert_eq!(RectLocation::center_left(), -RectLocation::center_right());
        assert_eq!(RectLocation::center(), -RectLocation::center());
    }

    #[test]
    fn rect_location_point_from_rect() {
        let left = -1.0;
        let top = 4.0;
        let right = 3.0;
        let bottom = -2.0;
        let center_x = (left + right) / 2.0;
        let center_y = (top + bottom) / 2.0;
        let rect = Rect::new(top, right, bottom, left);

        let top_left = RectLocation::top_left().point_from_rect(rect);
        assert_eq!(top_left.x, left);
        assert_eq!(top_left.y, top);
        let top_center = RectLocation::top_center().point_from_rect(rect);
        assert_eq!(top_center.x, center_x);
        assert_eq!(top_center.y, top);
        let top_right = RectLocation::top_right().point_from_rect(rect);
        assert_eq!(top_right.x, right);
        assert_eq!(top_right.y, top);

        let center_left = RectLocation::center_left().point_from_rect(rect);
        assert_eq!(center_left.x, left);
        assert_eq!(center_left.y, center_y);
        let center = RectLocation::center().point_from_rect(rect);
        assert_eq!(center.x, center_x);
        assert_eq!(center.y, center_y);
        let center_right = RectLocation::center_right().point_from_rect(rect);
        assert_eq!(center_right.x, right);
        assert_eq!(center_right.y, center_y);

        let bottom_left = RectLocation::bottom_left().point_from_rect(rect);
        assert_eq!(bottom_left.x, left);
        assert_eq!(bottom_left.y, bottom);
        let bottom_center = RectLocation::bottom_center().point_from_rect(rect);
        assert_eq!(bottom_center.x, center_x);
        assert_eq!(bottom_center.y, bottom);
        let bottom_right = RectLocation::bottom_right().point_from_rect(rect);
        assert_eq!(bottom_right.x, right);
        assert_eq!(bottom_right.y, bottom);
    }

    #[test]
    fn rect_position_translation() {
        let left = -1.0;
        let top = 4.0;
        let right = 3.0;
        let bottom = -2.0;
        let rect = Rect::new(top, right, bottom, left);

        let width_offset: Vector<f64> = Vector::new(rect.width(), 0.0);
        let height_offset: Vector<f64> = Vector::new(0.0, rect.height());
        let size_offset = width_offset + height_offset;
        assert_eq!(
            (RectPosition::bottom_left_from_rect(rect) + size_offset).point,
            RectPosition::top_right_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::center_right_from_rect(rect) - width_offset).point,
            RectPosition::center_left_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::bottom_left_from_rect(rect) + height_offset + width_offset).point,
            RectPosition::top_right_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::center_from_rect(rect) - height_offset / 2.0).point,
            RectPosition::bottom_center_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::bottom_center_from_rect(rect) + height_offset).point,
            RectPosition::top_center_from_rect(rect).point
        );
    }

    #[test]
    fn rect_position_rect_from_size() {
        let top = 4.0;
        let right = 3.0;
        let bottom = -2.0;
        let left = -1.0;
        let rect = Rect::new(top, right, bottom, left);

        assert_eq!(
            RectPosition::top_left_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::top_center_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::top_right_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_left_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_right_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).to_rect(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).to_rect(rect.size()),
            rect
        );
    }
}
