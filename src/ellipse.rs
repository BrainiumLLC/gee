use crate::{Angle, Point, Rect, Size, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T = f32> {
    center: Point<T>,
    radius: Size<T>,
}

impl<T: en::Num> Default for Ellipse<T> {
    fn default() -> Self {
        Self::unit()
    }
}

impl<T: en::Num> Ellipse<T> {
    pub fn new(center: Point<T>, radius: Size<T>) -> Self {
        Self { center, radius }
    }

    pub fn unit() -> Self {
        Self::new(Point::zero(), Size::square(T::one()))
    }

    pub fn zero() -> Self {
        Self::from_radius(Size::zero())
    }

    pub fn from_radius(radius: Size<T>) -> Self {
        Self::default().with_radius(radius)
    }

    pub fn from_center(center: Point<T>) -> Self {
        Self::default().with_center(center)
    }

    pub fn with_radius(mut self, radius: Size<T>) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_center(mut self, center: Point<T>) -> Self {
        self.center = center;
        self
    }

    pub fn add_radius(self, by: Size<T>) -> Self {
        self.map_radius(move |radius: Size<T>| radius + by)
    }

    pub fn scale_radius(self, coeff: Vector<T>) -> Self {
        self.map_radius(move |radius: Size<T>| radius.scale(coeff))
    }

    pub fn center(&self) -> Point<T> {
        self.center
    }

    pub fn radius(&self) -> Size<T> {
        self.radius
    }

    pub fn radius_squared(&self) -> Size<T> {
        self.radius * self.radius.to_vector()
    }

    pub fn contains(&self, point: Point<T>) -> bool {
        let offset = point - self.center;
        let offset_squared = offset * offset;
        let r_squared = self.radius_squared();
        offset_squared.dx / r_squared.width() + offset_squared.dy / r_squared.height() <= T::one()
    }

    pub fn bounding_rect(&self) -> Rect<T> {
        let offset = self.radius.to_vector();
        let top_left = self.center - offset;
        let bottom_right = self.center + offset;
        Rect::from_points(top_left, bottom_right)
    }

    pub fn arc_points(
        &self,
        steps: u32,
        start_angle: Angle<T>,
        end_angle: Angle<T>,
    ) -> impl Iterator<Item = Point<T>> + Clone
    where
        T: en::Float,
    {
        let radius = self.radius;
        let center = self.center;
        let increment = (end_angle - start_angle) / en::cast(steps);
        (0..steps).map(move |index| {
            let unit = (increment * en::cast(index) + start_angle).unit_vector();
            center + unit.scaled(radius)
        })
    }

    pub fn ellipse_points(
        &self,
        steps: u32,
        start_angle: Angle<T>,
    ) -> impl Iterator<Item = Point<T>> + Clone
    where
        T: en::Float,
    {
        self.arc_points(steps, start_angle, start_angle + Angle::TAU())
    }

    pub fn map<U: en::Num>(
        self,
        f: impl FnOnce(Point<T>, Size<T>) -> (Point<U>, Size<U>),
    ) -> Ellipse<U> {
        let (center, radius) = f(self.center, self.radius);
        Ellipse::new(center, radius)
    }

    pub fn map_center(self, f: impl FnOnce(Point<T>) -> Point<T>) -> Self {
        self.map(move |center, radius| (f(center), radius))
    }

    pub fn map_radius(self, f: impl FnOnce(Size<T>) -> Size<T>) -> Self {
        self.map(move |center, radius| (center, f(radius)))
    }

    pub fn cast<U: en::Num>(self) -> Ellipse<U> {
        self.map(move |center, radius| {
            (
                center.cast(),
                Size::new(en::cast(radius.width()), en::cast(radius.height())),
            )
        })
    }

    impl_casts!(Ellipse);
}

impl<T: en::Num> Add<Vector<T>> for Ellipse<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Ellipse::new(self.center + rhs, self.radius)
    }
}

impl<T: en::Num> AddAssign<Vector<T>> for Ellipse<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub<Vector<T>> for Ellipse<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Ellipse::new(self.center + rhs, self.radius)
    }
}

impl<T: en::Num> SubAssign<Vector<T>> for Ellipse<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs
    }
}
