use crate::{Angle, Point, Rect, Vector};
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

    pub fn create_scale(x: T, y: T) -> Self {
        Self::row_major(x, T::zero(), T::zero(), y, T::zero(), T::zero())
    }

    pub fn create_rotation(theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        let (sin, cos) = theta.sin_cos();
        Self::row_major(cos, -sin, sin, cos, T::zero(), T::zero())
    }

    pub fn create_translation(x: T, y: T) -> Self {
        Self::row_major(T::one(), T::zero(), T::zero(), T::one(), x, y)
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn transform_point(&self, point: &Point<T>) -> Point<T> {
        self.transform_vector(&point.to_vector()).to_point()
    }

    pub fn transform_vector(&self, vector: &Vector<T>) -> Vector<T> {
        Vector::new(
            vector.dx * self.m11 + vector.dy * self.m21,
            vector.dx * self.m12 + vector.dy * self.m22,
        )
    }

    pub fn transform_rect(&self, rect: &Rect<T>) -> Rect<T>
    where
        T: en::Num,
    {
        Rect::from_points_iter(&[
            self.transform_point(&rect.top_left()),
            self.transform_point(&rect.top_right()),
            self.transform_point(&rect.bottom_left()),
            self.transform_point(&rect.bottom_right()),
        ])
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
        self.post_mul(&Self::create_translation(x, y))
    }

    pub fn pre_translate(&self, x: T, y: T) -> Self {
        self.pre_mul(&Self::create_translation(x, y))
    }

    pub fn post_scale(&self, x: T, y: T) -> Self {
        self.post_mul(&Self::create_scale(x, y))
    }

    pub fn pre_scale(&self, x: T, y: T) -> Self {
        self.pre_mul(&Self::create_scale(x, y))
    }

    pub fn post_rotate(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.post_mul(&Self::create_rotation(theta))
    }

    pub fn pre_rotate(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.pre_mul(&Self::create_rotation(theta))
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

#[cfg(feature = "euclid")]
impl<T: en::Num, Src, Dst> From<euclid::Transform2D<T, Src, Dst>> for Transform<T> {
    fn from(transform: euclid::Transform2D<T, Src, Dst>) -> Self {
        Self::row_major(
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
impl<T: en::Num, Src, Dst> Into<euclid::Transform2D<T, Src, Dst>> for Transform<T> {
    #[rustfmt::skip]
    fn into(self) -> euclid::Transform2D<T, Src, Dst> {
        euclid::Transform2D::row_major(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + en::Num> From<nalgebra_glm::TMat3x2<T>> for Transform<T> {
    fn from(transform: nalgebra_glm::TMat3x2<T>) -> Self {
        Self::row_major(
            transform.m11,
            transform.m12,
            transform.m21,
            transform.m22,
            transform.m31,
            transform.m32,
        )
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + en::Num> Into<nalgebra_glm::TMat3x2<T>> for Transform<T> {
    #[rustfmt::skip]
    fn into(self) -> nalgebra_glm::TMat3x2<T> {
        nalgebra_glm::mat3x2(
            self.m11, self.m12,
            self.m21, self.m22,
            self.m31, self.m32,
        )
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + en::Num> Into<nalgebra_glm::TMat3<T>> for Transform<T> {
    #[rustfmt::skip]
    fn into(self) -> nalgebra_glm::TMat3<T> {
        let (_0, _1) = (T::zero(), T::one());
        nalgebra_glm::mat3(
            self.m11, self.m12, _0,
            self.m21, self.m22, _0,
            self.m31, self.m32, _1,
        )
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + en::Num> Into<nalgebra_glm::TMat4<T>> for Transform<T> {
    #[rustfmt::skip]
    fn into(self) -> nalgebra_glm::TMat4<T> {
        let (_0, _1) = (T::zero(), T::one());
        // Using the provided `nalgebra_glm::mat3_to_mat4` function would
        // require adding a `nalgebra_glm::Number` bound...
        nalgebra_glm::mat4(
            self.m11, self.m12, _0, _0,
            self.m21, self.m22, _0, _0,
            self.m31, self.m32, _1, _0,
            _0, _0, _0, _1,
        )
    }
}

#[cfg(feature = "nalgebra-glm")]
impl<T: 'static + en::Num> Transform<T> {
    pub fn from_glm_mat3x2(transform: nalgebra_glm::TMat3x2<T>) -> Self {
        Self::from(transform)
    }

    pub fn to_glm_mat3x2(self) -> nalgebra_glm::TMat3x2<T> {
        self.into()
    }

    pub fn to_glm_mat3(self) -> nalgebra_glm::TMat3<T> {
        self.into()
    }

    pub fn to_glm_mat4(self) -> nalgebra_glm::TMat4<T> {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn rotation() {
        let original = Vector::new(1.0, 1.0).normalized();
        let rotated =
            Transform::create_rotation(Angle::from_degrees(-45.0)).transform_vector(&original);
        assert_approx_eq!(rotated.dx, 1.0);
        assert_approx_eq!(rotated.dy, 0.0);

        let rotated =
            Transform::create_rotation(Angle::from_degrees(45.0)).transform_vector(&original);
        assert_approx_eq!(rotated.dx, 0.0);
        assert_approx_eq!(rotated.dy, 1.0);
    }
}
