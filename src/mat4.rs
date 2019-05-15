#[cfg(feature = "euclid")]
use euclid::Transform3D;
use num_traits::{Float, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat4<T> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
}

impl<T> Mat4<T> {
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
}

impl<T: One + Zero> Default for Mat4<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: One + Zero> Mat4<T> {
    pub fn identity() -> Self {
        Self::row_major(
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),

            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),

            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),

            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        )
    }
}

impl<T: Float> Mat4<T> {
    pub fn ortho(
        left: T,
        right: T,
        bottom: T,
        top: T,
        near: T,
        far: T,
    ) -> Self {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        let (_0, _1): (T, T) = (Zero::zero(), One::one());
        let _2 = _1 + _1;
        let sx = _2 / (right - left);
        let sy = _2 / (top - bottom);
        let sz = -_2 / (far - near);
        Self::row_major(
            sx, _0, _0, _0,
            _0, sy, _0, _0,
            _0, _0, sz, _0,
            tx, ty, tz, _1,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T> From<Transform3D<T>> for Mat4<T> {
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
impl<T: Copy> Into<Transform3D<T>> for Mat4<T> {
    fn into(self) -> Transform3D<T> {
        Transform3D::row_major(
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        )
    }
}
