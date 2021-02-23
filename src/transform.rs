use crate::{Angle, Point, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

impl<T: en::Num> Default for Transform<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: en::Num> Transform<T> {
    pub fn row_major(m11: T, m12: T, m21: T, m22: T, m31: T, m32: T) -> Self {
        Self {
            m11,
            m12,
            m21,
            m22,
            m31,
            m32,
        }
    }

    pub fn identity() -> Self {
        Self::row_major(
            T::one(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
        )
    }

    pub fn from_scale(x: T, y: T) -> Self {
        Self::row_major(x, T::zero(), T::zero(), y, T::zero(), T::zero())
    }

    pub fn from_scale_vector(scale: Vector<T>) -> Self {
        Self::from_scale(scale.dx, scale.dy)
    }

    pub fn from_rotation(theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        let (sin, cos) = theta.sin_cos();
        Self::row_major(cos, sin, -sin, cos, T::zero(), T::zero())
    }

    pub fn from_rotation_with_fixed_point(theta: Angle<T>, point: Point<T>) -> Self
    where
        T: en::Float,
    {
        Self::from_rotation(theta)
            .pre_translate(-point.x, -point.y)
            .post_translate(point.x, point.y)
    }

    pub fn from_translation(x: T, y: T) -> Self {
        Self::row_major(T::one(), T::zero(), T::zero(), T::one(), x, y)
    }

    pub fn from_translation_vector(translation: Vector<T>) -> Self {
        Self::from_translation(translation.dx, translation.dy)
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn post_mul(&self, mat: &Self) -> Self {
        Self::row_major(
            self.m11 * mat.m11 + self.m12 * mat.m21,
            self.m11 * mat.m12 + self.m12 * mat.m22,
            self.m21 * mat.m11 + self.m22 * mat.m21,
            self.m21 * mat.m12 + self.m22 * mat.m22,
            self.m31 * mat.m11 + self.m32 * mat.m21 + mat.m31,
            self.m31 * mat.m12 + self.m32 * mat.m22 + mat.m32,
        )
    }

    pub fn pre_mul(&self, mat: &Self) -> Self {
        mat.post_mul(self)
    }

    pub fn post_translate(&self, x: T, y: T) -> Self {
        self.post_mul(&Self::from_translation(x, y))
    }

    pub fn post_translate_vector(&self, translation: Vector<T>) -> Self {
        self.post_mul(&Self::from_translation_vector(translation))
    }

    pub fn pre_translate(&self, x: T, y: T) -> Self {
        self.pre_mul(&Self::from_translation(x, y))
    }

    pub fn pre_translate_vector(&self, translation: Vector<T>) -> Self {
        self.pre_mul(&Self::from_translation_vector(translation))
    }

    pub fn post_scale(&self, x: T, y: T) -> Self {
        self.post_mul(&Self::from_scale(x, y))
    }

    pub fn post_scale_vector(&self, scale: Vector<T>) -> Self {
        self.post_mul(&Self::from_scale_vector(scale))
    }

    pub fn pre_scale(&self, x: T, y: T) -> Self {
        self.pre_mul(&Self::from_scale(x, y))
    }

    pub fn pre_scale_vector(&self, scale: Vector<T>) -> Self {
        self.pre_mul(&Self::from_scale_vector(scale))
    }

    pub fn post_rotate(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.post_mul(&Self::from_rotation(theta))
    }

    pub fn pre_rotate(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.pre_mul(&Self::from_rotation(theta))
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();

        let _0: T = T::zero();
        let _1: T = T::one();

        if det == _0 {
            return None;
        }

        let inv_det = _1 / det;
        Some(Self::row_major(
            inv_det * self.m22,
            inv_det * (_0 - self.m12),
            inv_det * (_0 - self.m21),
            inv_det * self.m11,
            inv_det * (self.m21 * self.m32 - self.m22 * self.m31),
            inv_det * (self.m31 * self.m12 - self.m11 * self.m32),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_approx_eq, Vector};

    #[test]
    fn rotation() {
        let original = Vector::new(1.0, 1.0).normalized();
        let rotated = original.transform(Transform::from_rotation(Angle::from_degrees(-45.0)));
        assert_approx_eq!(rotated.dx, 1.0);
        assert_approx_eq!(rotated.dy, 0.0);

        let rotated = original.transform(Transform::from_rotation(Angle::from_degrees(45.0)));
        assert_approx_eq!(rotated.dx, 0.0);
        assert_approx_eq!(rotated.dy, 1.0);
    }
}
