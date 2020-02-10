use crate::{OrdinaryNum, Point, Rect, Size, Vec2};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
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
    pub vertical:   VerticalLocation,
}

impl Neg for RectLocation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            horizontal: -self.horizontal,
            vertical:   -self.vertical,
        }
    }
}

impl RectLocation {
    pub fn top_left() -> Self {
        Self {
            horizontal: HorizontalLocation::Left,
            vertical:   VerticalLocation::Top,
        }
    }
    pub fn top_center() -> Self {
        Self {
            horizontal: HorizontalLocation::Center,
            vertical:   VerticalLocation::Top,
        }
    }
    pub fn top_right() -> Self {
        Self {
            horizontal: HorizontalLocation::Right,
            vertical:   VerticalLocation::Top,
        }
    }
    pub fn center_left() -> Self {
        Self {
            horizontal: HorizontalLocation::Left,
            vertical:   VerticalLocation::Center,
        }
    }
    pub fn center() -> Self {
        Self {
            horizontal: HorizontalLocation::Center,
            vertical:   VerticalLocation::Center,
        }
    }
    pub fn center_right() -> Self {
        Self {
            horizontal: HorizontalLocation::Right,
            vertical:   VerticalLocation::Center,
        }
    }
    pub fn bottom_left() -> Self {
        Self {
            horizontal: HorizontalLocation::Left,
            vertical:   VerticalLocation::Bottom,
        }
    }
    pub fn bottom_center() -> Self {
        Self {
            horizontal: HorizontalLocation::Center,
            vertical:   VerticalLocation::Bottom,
        }
    }
    pub fn bottom_right() -> Self {
        Self {
            horizontal: HorizontalLocation::Right,
            vertical:   VerticalLocation::Bottom,
        }
    }

    pub fn point_from_rect<T: OrdinaryNum>(&self, rect: Rect<T>) -> Point<T> {
        let x = match self.horizontal {
            HorizontalLocation::Left => rect.left(),
            HorizontalLocation::Center => rect.center_x(),
            HorizontalLocation::Right => rect.right(),
        };
        let y = match self.vertical {
            VerticalLocation::Top => rect.top(),
            VerticalLocation::Center => rect.center_y(),
            VerticalLocation::Bottom => rect.bottom(),
        };
        Point::new(x, y)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RectPosition<T> {
    pub location: RectLocation,
    pub point:    Point<T>,
}

impl<T: OrdinaryNum> RectPosition<T> {
    pub fn top_left(point: Point<T>) -> Self {
        Self {
            location: RectLocation::top_left(),
            point,
        }
    }
    pub fn top_center(point: Point<T>) -> Self {
        Self {
            location: RectLocation::top_center(),
            point,
        }
    }
    pub fn top_right(point: Point<T>) -> Self {
        Self {
            location: RectLocation::top_right(),
            point,
        }
    }

    pub fn center_left(point: Point<T>) -> Self {
        Self {
            location: RectLocation::center_left(),
            point,
        }
    }
    pub fn center(point: Point<T>) -> Self {
        Self {
            location: RectLocation::center(),
            point,
        }
    }
    pub fn center_right(point: Point<T>) -> Self {
        Self {
            location: RectLocation::center_right(),
            point,
        }
    }

    pub fn bottom_left(point: Point<T>) -> Self {
        Self {
            location: RectLocation::bottom_left(),
            point,
        }
    }
    pub fn bottom_center(point: Point<T>) -> Self {
        Self {
            location: RectLocation::bottom_center(),
            point,
        }
    }
    pub fn bottom_right(point: Point<T>) -> Self {
        Self {
            location: RectLocation::bottom_right(),
            point,
        }
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

    pub fn left_with_width(&self, width: T) -> T {
        use HorizontalLocation::*;
        assert!(width >= T::zero(), "invalid value for width: {:?}", width);
        match self.location.horizontal {
            Left => self.point.x,
            Center => self.point.x - width.halved(),
            Right => self.point.x - width,
        }
    }
    pub fn center_x_with_width(&self, width: T) -> T {
        use HorizontalLocation::*;
        assert!(width >= T::zero(), "invalid value for width: {:?}", width);
        match self.location.horizontal {
            Left => self.point.x + width.halved(),
            Center => self.point.x,
            Right => self.point.x - width.halved(),
        }
    }
    pub fn right_with_width(&self, width: T) -> T {
        use HorizontalLocation::*;
        assert!(width >= T::zero(), "invalid value for width: {:?}", width);
        match self.location.horizontal {
            Left => self.point.x + width,
            Center => self.point.x + width.halved(),
            Right => self.point.x,
        }
    }

    pub fn top_with_height(&self, height: T) -> T {
        use VerticalLocation::*;
        assert!(
            height >= T::zero(),
            "invalid value for height: {:?}",
            height
        );
        match self.location.vertical {
            Top => self.point.y,
            Center => self.point.y - height.halved(),
            Bottom => self.point.y - height,
        }
    }
    pub fn center_y_with_height(&self, height: T) -> T {
        use VerticalLocation::*;
        assert!(
            height >= T::zero(),
            "invalid value for height: {:?}",
            height
        );
        match self.location.vertical {
            Top => self.point.y + height.halved(),
            Center => self.point.y,
            Bottom => self.point.y - height.halved(),
        }
    }
    pub fn bottom_with_height(&self, height: T) -> T {
        use VerticalLocation::*;
        assert!(
            height >= T::zero(),
            "invalid value for height: {:?}",
            height
        );
        match self.location.vertical {
            Top => self.point.y + height,
            Center => self.point.y + height.halved(),
            Bottom => self.point.y,
        }
    }

    pub fn rect_with_size(&self, size: Size<T>) -> Rect<T> {
        let width = size.width();
        let height = size.height();
        Rect::new(
            self.top_with_height(height),
            self.right_with_width(width),
            self.bottom_with_height(height),
            self.left_with_width(width),
        )
    }
}

impl<T: Add<RHS>, RHS> Add<Vec2<RHS>> for RectPosition<T> {
    type Output = RectPosition<T::Output>;
    fn add(self, rhs: Vec2<RHS>) -> Self::Output {
        RectPosition {
            location: self.location,
            point:    self.point + rhs,
        }
    }
}

impl<T: AddAssign<RHS>, RHS> AddAssign<Vec2<RHS>> for RectPosition<T> {
    fn add_assign(&mut self, rhs: Vec2<RHS>) {
        self.point += rhs;
    }
}

impl<T: Sub<RHS>, RHS> Sub<Vec2<RHS>> for RectPosition<T> {
    type Output = RectPosition<T::Output>;
    fn sub(self, rhs: Vec2<RHS>) -> Self::Output {
        RectPosition {
            location: self.location,
            point:    self.point - rhs,
        }
    }
}

impl<T: SubAssign<RHS>, RHS> SubAssign<Vec2<RHS>> for RectPosition<T> {
    fn sub_assign(&mut self, rhs: Vec2<RHS>) {
        self.point -= rhs;
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
        let top = -2.0;
        let right = 3.0;
        let bottom = 4.0;
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
        let top = -2.0;
        let right = 3.0;
        let bottom = 4.0;
        let rect = Rect::new(top, right, bottom, left);

        let width_offset: Vec2<f64> = Vec2::new(rect.width(), 0.0);
        let height_offset: Vec2<f64> = Vec2::new(0.0, rect.height());
        let size_offset = width_offset + height_offset;
        assert_eq!(
            (RectPosition::top_left_from_rect(rect) + size_offset).point,
            RectPosition::bottom_right_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::center_right_from_rect(rect) - width_offset).point,
            RectPosition::center_left_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::bottom_left_from_rect(rect) - height_offset + width_offset).point,
            RectPosition::top_right_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::center_from_rect(rect) + height_offset / 2.0).point,
            RectPosition::bottom_center_from_rect(rect).point
        );
        assert_eq!(
            (RectPosition::bottom_center_from_rect(rect) - height_offset).point,
            RectPosition::top_center_from_rect(rect).point
        );
    }

    #[test]
    fn rect_position_rect_from_size() {
        let top = -2.0;
        let right = 3.0;
        let bottom = 4.0;
        let left = -1.0;
        let rect = Rect::new(top, right, bottom, left);

        assert_eq!(
            RectPosition::top_left_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::top_center_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::top_right_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_left_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::center_right_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
        assert_eq!(
            RectPosition::bottom_left_from_rect(rect).rect_with_size(rect.size()),
            rect
        );
    }
}
