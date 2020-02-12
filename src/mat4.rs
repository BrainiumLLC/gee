use crate::{Angle, OrdinaryFloat, OrdinaryNum, Vec3};
#[cfg(feature = "euclid")]
use euclid::Transform3D;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Neg;

pub type Mat4x4<T> = Mat4<T>;

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat4<T> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
}

impl<T: OrdinaryNum> Default for Mat4<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: OrdinaryNum> Mat4<T> {
    #[rustfmt::skip]
    pub fn row_major(
        m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T,
    ) -> Self {
        Self {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        }
    }

    pub fn identity() -> Self {
        Self::row_major(
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        )
    }

    pub fn create_scale(x: T, y: T, z: T) -> Self {
        Self::row_major(
            x,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            y,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            z,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        )
    }

    pub fn create_translation(x: T, y: T, z: T) -> Self {
        Self::row_major(
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            x,
            y,
            z,
            T::one(),
        )
    }

    pub fn create_rotation(
        Vec3 {
            dx: x,
            dy: y,
            dz: z,
        }: Vec3<T>,
        theta: Angle<T>,
    ) -> Self
    where
        T: OrdinaryFloat,
    {
        let (_0, _1): (T, T) = (T::zero(), T::one());
        let _2 = _1 + _1;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let half_theta = theta.radians / _2;
        let sc = half_theta.sin() * half_theta.cos();
        let sq = half_theta.sin() * half_theta.sin();

        Self::row_major(
            _1 - _2 * (yy + zz) * sq,
            _2 * (x * y * sq - z * sc),
            _2 * (x * z * sq + y * sc),
            _0,
            _2 * (x * y * sq + z * sc),
            _1 - _2 * (xx + zz) * sq,
            _2 * (y * z * sq - x * sc),
            _0,
            _2 * (x * z * sq - y * sc),
            _2 * (y * z * sq + x * sc),
            _1 - _2 * (xx + yy) * sq,
            _0,
            _0,
            _0,
            _0,
            _1,
        )
    }

    pub fn post_mul(&self, mat: &Self) -> Self {
        Self::row_major(
            self.m11 * mat.m11 + self.m12 * mat.m21 + self.m13 * mat.m31 + self.m14 * mat.m41,
            self.m11 * mat.m12 + self.m12 * mat.m22 + self.m13 * mat.m32 + self.m14 * mat.m42,
            self.m11 * mat.m13 + self.m12 * mat.m23 + self.m13 * mat.m33 + self.m14 * mat.m43,
            self.m11 * mat.m14 + self.m12 * mat.m24 + self.m13 * mat.m34 + self.m14 * mat.m44,
            self.m21 * mat.m11 + self.m22 * mat.m21 + self.m23 * mat.m31 + self.m24 * mat.m41,
            self.m21 * mat.m12 + self.m22 * mat.m22 + self.m23 * mat.m32 + self.m24 * mat.m42,
            self.m21 * mat.m13 + self.m22 * mat.m23 + self.m23 * mat.m33 + self.m24 * mat.m43,
            self.m21 * mat.m14 + self.m22 * mat.m24 + self.m23 * mat.m34 + self.m24 * mat.m44,
            self.m31 * mat.m11 + self.m32 * mat.m21 + self.m33 * mat.m31 + self.m34 * mat.m41,
            self.m31 * mat.m12 + self.m32 * mat.m22 + self.m33 * mat.m32 + self.m34 * mat.m42,
            self.m31 * mat.m13 + self.m32 * mat.m23 + self.m33 * mat.m33 + self.m34 * mat.m43,
            self.m31 * mat.m14 + self.m32 * mat.m24 + self.m33 * mat.m34 + self.m34 * mat.m44,
            self.m41 * mat.m11 + self.m42 * mat.m21 + self.m43 * mat.m31 + self.m44 * mat.m41,
            self.m41 * mat.m12 + self.m42 * mat.m22 + self.m43 * mat.m32 + self.m44 * mat.m42,
            self.m41 * mat.m13 + self.m42 * mat.m23 + self.m43 * mat.m33 + self.m44 * mat.m43,
            self.m41 * mat.m14 + self.m42 * mat.m24 + self.m43 * mat.m34 + self.m44 * mat.m44,
        )
    }

    pub fn pre_mul(&self, mat: &Self) -> Self {
        mat.post_mul(self)
    }

    pub fn post_scale(&self, x: T, y: T, z: T) -> Self {
        self.post_mul(&Self::create_scale(x, y, z))
    }

    pub fn pre_scale(&self, x: T, y: T, z: T) -> Self {
        self.pre_mul(&Self::create_scale(x, y, z))
    }

    pub fn post_translate(self, x: T, y: T, z: T) -> Self {
        self.post_mul(&Self::create_translation(x, y, z))
    }

    pub fn pre_translate(self, x: T, y: T, z: T) -> Self {
        self.pre_mul(&Self::create_translation(x, y, z))
    }

    pub fn post_rotate(self, axis: Vec3<T>, theta: Angle<T>) -> Self
    where
        T: OrdinaryFloat,
    {
        self.post_mul(&Self::create_rotation(axis, theta))
    }

    pub fn pre_rotate(self, axis: Vec3<T>, theta: Angle<T>) -> Self
    where
        T: OrdinaryFloat,
    {
        self.pre_mul(&Self::create_rotation(axis, theta))
    }

    pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self
    where
        T: Neg<Output = T>,
    {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        let (_0, _1): (T, T) = (T::zero(), T::one());
        let _2 = _1 + _1;
        let sx = _2 / (right - left);
        let sy = _2 / (top - bottom);
        let sz = -_2 / (far - near);
        Self::row_major(
            sx, _0, _0, _0, _0, sy, _0, _0, _0, _0, sz, _0, tx, ty, tz, _1,
        )
    }

    pub fn persp(aspect: T, fov: Angle<T>, near: T, far: T) -> Self
    where
        T: OrdinaryFloat,
    {
        let (_0, _1): (T, T) = (T::zero(), T::one());
        let _2 = _1 + _1;
        let f = (fov.radians / _2).tan().recip();
        let depth = near - far;
        Self::row_major(
            f / aspect,
            _0,
            _0,
            _0,
            _0,
            -f,
            _0,
            _0,
            _0,
            _0,
            far / depth,
            -_1,
            _0,
            _0,
            near * far / depth,
            _0,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> From<Transform3D<T>> for Mat4<T> {
    fn from(transform: Transform3D<T>) -> Self {
        Self::row_major(
            transform.m11,
            transform.m12,
            transform.m13,
            transform.m14,
            transform.m21,
            transform.m22,
            transform.m23,
            transform.m24,
            transform.m31,
            transform.m32,
            transform.m33,
            transform.m34,
            transform.m41,
            transform.m42,
            transform.m43,
            transform.m44,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T: OrdinaryNum> Into<Transform3D<T>> for Mat4<T> {
    #[rustfmt::skip]
    fn into(self) -> Transform3D<T> {
        Transform3D::row_major(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }
}
