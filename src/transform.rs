use crate::{point::Point, rect::Rect, vector::Vector};
#[cfg(feature = "euclid")]
use euclid::Transform2D;
use num_traits::{Float, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Transform<T> {
    pub m11: T,
    pub m12: T,
    pub m21: T,
    pub m22: T,
    pub m31: T,
    pub m32: T,
}

impl<T> Transform<T> {
    pub fn row_major(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Self {
        Transform {
            m11,
            m12,
            m21,
            m22,
            m31,
            m32,
        }
    }
}

impl<T: Zero> Transform<T> {
    pub fn create_scale(x: T, y: T) -> Self {
        Transform::row_major(x, T::zero(), T::zero(), y, T::zero(), T::zero())
    }
}

impl<T: Float> Transform<T> {
    pub fn create_rotation(theta: T) -> Self {
        let _0 = Zero::zero();
        let (sin, cos) = theta.sin_cos();
        Transform::row_major(cos, _0 - sin, sin, cos, _0, _0)
    }
}

impl<T: One + Zero> Default for Transform<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: One + Zero> Transform<T> {
    pub fn identity() -> Self {
        Transform::row_major(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
        )
    }

    pub fn create_translation(x: T, y: T) -> Self {
        Transform::row_major(One::one(), Zero::zero(), Zero::zero(), One::one(), x, y)
    }
}

impl<T: Copy> Transform<T> {
    pub fn transform_point<U>(
        &self,
        point: &Point<U>,
    ) -> Point<<<U::Output as Add>::Output as Add<T>>::Output>
    where
        U: Mul<T> + Copy,
        U::Output: Add,
        <U::Output as Add>::Output: Add<T>,
    {
        Point::new(
            point.x * self.m11 + point.y * self.m21 + self.m31,
            point.x * self.m12 + point.y * self.m22 + self.m32,
        )
    }

    pub fn transform_vector<U>(&self, vector: &Vector<U>) -> Vector<<U::Output as Add>::Output>
    where
        U: Mul<T> + Copy,
        U::Output: Add,
    {
        Vector::new(
            vector.dx * self.m11 + vector.dy * self.m21,
            vector.dx * self.m12 + vector.dy * self.m22,
        )
    }

    pub fn transform_rect<U>(
        &self,
        rect: &Rect<U>,
    ) -> Rect<<<U::Output as Add>::Output as Add<T>>::Output>
    where
        U: Mul<T> + Copy,
        U::Output: Add,
        <U::Output as Add>::Output: Add<T>,
        <<U::Output as Add>::Output as Add<T>>::Output: Copy + PartialOrd + Zero,
    {
        Rect::from_points_iter(&[
            self.transform_point(&rect.top_left()),
            self.transform_point(&rect.top_right()),
            self.transform_point(&rect.bottom_left()),
            self.transform_point(&rect.bottom_right()),
        ])
    }

    pub fn post_mul<U>(&self, mat: &Transform<U>) -> Transform<<T::Output as Add>::Output>
    where
        U: Copy,
        T: Mul<U>,
        T::Output: Add,
        <T::Output as Add>::Output: Add<U, Output = <T::Output as Add>::Output>,
    {
        Transform::row_major(
            self.m11 * mat.m11 + self.m12 * mat.m21,
            self.m11 * mat.m12 + self.m12 * mat.m22,
            self.m21 * mat.m11 + self.m22 * mat.m21,
            self.m21 * mat.m12 + self.m22 * mat.m22,
            self.m31 * mat.m11 + self.m32 * mat.m21 + mat.m31,
            self.m31 * mat.m12 + self.m32 * mat.m22 + mat.m32,
        )
    }

    pub fn post_translate<U>(&self, vec: Vector<U>) -> Transform<<T::Output as Add>::Output>
    where
        U: Copy + Zero + One,
        T: Mul<U>,
        T::Output: Add,
        <T::Output as Add>::Output: Add<U, Output = <T::Output as Add>::Output>,
    {
        self.post_mul(&Transform::create_translation(vec.dx, vec.dy))
    }

    pub fn post_scale<U>(&self, x: U, y: U) -> Transform<<T::Output as Add>::Output>
    where
        U: Copy + Zero,
        T: Mul<U>,
        T::Output: Add,
        <T::Output as Add>::Output: Add<U, Output = <T::Output as Add>::Output>,
    {
        self.post_mul(&Transform::create_scale(x, y))
    }

    pub fn post_rotate<U: Float>(&self, theta: U) -> Transform<<T::Output as Add>::Output>
    where
        T: Mul<U>,
        T::Output: Add,
        <T::Output as Add>::Output: Add<U, Output = <T::Output as Add>::Output>,
    {
        self.post_mul(&Transform::create_rotation(theta))
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Transform2D<T>> for Transform<T> {
    fn from(transform: Transform2D<T>) -> Self {
        Transform::row_major(
            transform.m11,
            transform.m12,
            transform.m21,
            transform.m22,
            transform.m31,
            transform.m32,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T: Copy> Into<Transform2D<T>> for Transform<T> {
    fn into(self) -> Transform2D<T> {
        Transform2D::row_major(self.m11, self.m12, self.m21, self.m22, self.m31, self.m32)
    }
}
