use crate::{Angle, Point, Rect, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    center: Point<T>,
    radius: T,
}

impl<T: en::Num> Default for Circle<T> {
    fn default() -> Self {
        Self::unit()
    }
}

impl<T: en::Num> Circle<T> {
    pub fn new_unchecked(center: Point<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn try_new(center: Point<T>, radius: T) -> Option<Self> {
        if radius >= T::zero() {
            Some(Self::new_unchecked(center, radius))
        } else {
            None
        }
    }

    pub fn new(center: Point<T>, radius: T) -> Self {
        Self::try_new(center, radius).expect("radius is less than 0")
    }

    pub fn unit() -> Self {
        Self::with_radius(T::one())
    }

    pub fn zero() -> Self {
        Self::with_radius(T::zero())
    }

    pub fn with_radius(radius: T) -> Self {
        Self::new(Point::zero(), radius)
    }

    pub fn with_center(center: Point<T>) -> Self {
        Self::new(center, T::one())
    }

    pub fn center(&self) -> Point<T> {
        self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn added_radius(&self, by: T) -> Self {
        self.map_radius(move |radius: T| radius + by)
    }

    pub fn scaled_radius(&self, coeff: T) -> Self {
        self.map_radius(move |radius: T| radius * coeff)
    }

    pub fn bounding_rect(&self) -> Rect<T> {
        let radius_offset: Vector<T> = Vector::new(self.radius, self.radius);
        let top_left = self.center + radius_offset;
        let bottom_right = self.center + radius_offset;
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
            center + unit * radius
        })
    }

    pub fn circle_points(
        &self,
        steps: u32,
        start_angle: Angle<T>,
    ) -> impl Iterator<Item = Point<T>> + Clone
    where
        T: en::Float,
    {
        self.arc_points(steps, start_angle, start_angle + Angle::TAU())
    }

    pub fn map<U: en::Num>(self, f: impl FnOnce(Point<T>, T) -> (Point<U>, U)) -> Circle<U> {
        let (center, radius) = f(self.center, self.radius);
        Circle::new(center, radius)
    }

    pub fn map_center(self, f: impl FnOnce(Point<T>) -> Point<T>) -> Self {
        self.map(move |center, radius| (f(center), radius))
    }

    pub fn map_radius(self, f: impl FnOnce(T) -> T) -> Self {
        self.map(move |center, radius| (center, f(radius)))
    }

    pub fn cast<U: en::Num>(self) -> Circle<U> {
        self.map(move |center, radius| (center.cast(), en::cast(radius)))
    }

    impl_casts!(Circle);
}

impl<T: en::Num> Add<Vector<T>> for Circle<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        // radius unmodified
        Circle::new_unchecked(self.center + rhs, self.radius)
    }
}

impl<T: en::Num> AddAssign<Vector<T>> for Circle<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub<Vector<T>> for Circle<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        // radius unmodified
        Circle::new_unchecked(self.center + rhs, self.radius)
    }
}

impl<T: en::Num> SubAssign<Vector<T>> for Circle<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        *self = *self - rhs
    }
}
