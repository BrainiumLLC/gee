use crate::{Angle, Point, Quad, Rect, Size, Transform, Vector, Vector4d};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Transform3d<T = f32> {
    pub m11: T, pub m12: T, pub m13: T, pub m14: T,
    pub m21: T, pub m22: T, pub m23: T, pub m24: T,
    pub m31: T, pub m32: T, pub m33: T, pub m34: T,
    pub m41: T, pub m42: T, pub m43: T, pub m44: T,
}

impl<T: en::Num> Default for Transform3d<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: en::Num> Transform3d<T> {
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

    /// Creates a multiplicative identity matrix.
    #[rustfmt::skip]
    pub fn identity() -> Self {
        let (_0, _1) = (T::zero(), T::one());
        Self::row_major(
            _1, _0, _0, _0,
            _0, _1, _0, _0,
            _0, _0, _1, _0,
            _0, _0, _0, _1,
        )
    }

    pub fn from_scale(x: T, y: T, z: T) -> Self {
        Self {
            m11: x,
            m22: y,
            m33: z,
            ..Self::identity()
        }
    }

    pub fn from_translation(x: T, y: T, z: T) -> Self {
        Self {
            m41: x,
            m42: y,
            m43: z,
            ..Self::identity()
        }
    }

    /// Creates an orthographic projection matrix with all of those standard
    /// arguments you can never remember.
    pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self
    where
        T: en::Float,
    {
        let _2 = T::two();
        Self::from_scale(_2 / (right - left), _2 / (bottom - top), -_2 / (far - near)).post_mul(
            Self::from_translation(
                -(right + left) / (right - left),
                -(bottom + top) / (bottom - top),
                -(far + near) / (far - near),
            ),
        )
    }

    /// Creates an orthographic projection matrix with the defaults you almost
    /// always want.
    pub fn ortho_from_rect(rect: Rect<T>) -> Self
    where
        T: en::Float,
    {
        let (_0, _1) = (T::zero(), T::one());
        Self::ortho(
            rect.left(),
            rect.right(),
            rect.bottom(),
            rect.top(),
            _1,
            -_1,
        )
    }

    /// Creates a perspective projection matrix for whatever 3D side project
    /// you've been secretly making.
    #[rustfmt::skip]
    pub fn persp(size: Size<T>, fov: Angle<T>, near: T, far: T) -> Self
    where
        T: en::Float,
    {
        let (_0, _1) = (T::zero(), T::one());
        let f = fov.radians().halved().tan().recip();
        let depth = near - far;
        Self::row_major(
            f / size.aspect_ratio(), _0, _0, _0,
            _0, f, _0, _0,
            _0, _0, far / depth, -_1,
            _0, _0, near * far / depth, _0,
        )
    }

    /// Creates a new matrix by multiplying another matrix after this matrix.
    pub fn post_mul(self, mat: Self) -> Transform3d<T>
    where
        T: en::Float,
    {
        Transform3d::row_major(
            self.m11.mul_add(
                mat.m11,
                self.m12
                    .mul_add(mat.m21, self.m13.mul_add(mat.m31, self.m14 * mat.m41)),
            ),
            self.m11.mul_add(
                mat.m12,
                self.m12
                    .mul_add(mat.m22, self.m13.mul_add(mat.m32, self.m14 * mat.m42)),
            ),
            self.m11.mul_add(
                mat.m13,
                self.m12
                    .mul_add(mat.m23, self.m13.mul_add(mat.m33, self.m14 * mat.m43)),
            ),
            self.m11.mul_add(
                mat.m14,
                self.m12
                    .mul_add(mat.m24, self.m13.mul_add(mat.m34, self.m14 * mat.m44)),
            ),
            self.m21.mul_add(
                mat.m11,
                self.m22
                    .mul_add(mat.m21, self.m23.mul_add(mat.m31, self.m24 * mat.m41)),
            ),
            self.m21.mul_add(
                mat.m12,
                self.m22
                    .mul_add(mat.m22, self.m23.mul_add(mat.m32, self.m24 * mat.m42)),
            ),
            self.m21.mul_add(
                mat.m13,
                self.m22
                    .mul_add(mat.m23, self.m23.mul_add(mat.m33, self.m24 * mat.m43)),
            ),
            self.m21.mul_add(
                mat.m14,
                self.m22
                    .mul_add(mat.m24, self.m23.mul_add(mat.m34, self.m24 * mat.m44)),
            ),
            self.m31.mul_add(
                mat.m11,
                self.m32
                    .mul_add(mat.m21, self.m33.mul_add(mat.m31, self.m34 * mat.m41)),
            ),
            self.m31.mul_add(
                mat.m12,
                self.m32
                    .mul_add(mat.m22, self.m33.mul_add(mat.m32, self.m34 * mat.m42)),
            ),
            self.m31.mul_add(
                mat.m13,
                self.m32
                    .mul_add(mat.m23, self.m33.mul_add(mat.m33, self.m34 * mat.m43)),
            ),
            self.m31.mul_add(
                mat.m14,
                self.m32
                    .mul_add(mat.m24, self.m33.mul_add(mat.m34, self.m34 * mat.m44)),
            ),
            self.m41.mul_add(
                mat.m11,
                self.m42
                    .mul_add(mat.m21, self.m43.mul_add(mat.m31, self.m44 * mat.m41)),
            ),
            self.m41.mul_add(
                mat.m12,
                self.m42
                    .mul_add(mat.m22, self.m43.mul_add(mat.m32, self.m44 * mat.m42)),
            ),
            self.m41.mul_add(
                mat.m13,
                self.m42
                    .mul_add(mat.m23, self.m43.mul_add(mat.m33, self.m44 * mat.m43)),
            ),
            self.m41.mul_add(
                mat.m14,
                self.m42
                    .mul_add(mat.m24, self.m43.mul_add(mat.m34, self.m44 * mat.m44)),
            ),
        )
    }

    pub fn pre_mul(&self, mat: Self) -> Self
    where
        T: en::Float,
    {
        mat.post_mul(*self)
    }

    pub fn map<U: en::Num>(self, mut f: impl FnMut(T) -> U) -> Transform3d<U> {
        Transform3d::row_major(
            f(self.m11),
            f(self.m12),
            f(self.m13),
            f(self.m14),
            f(self.m21),
            f(self.m22),
            f(self.m23),
            f(self.m24),
            f(self.m31),
            f(self.m32),
            f(self.m33),
            f(self.m34),
            f(self.m41),
            f(self.m42),
            f(self.m43),
            f(self.m44),
        )
    }

    impl_casts_and_cast!(Transform3d);
}

impl<T: en::Num> Transform3d<T> {
    pub fn transform_vector(&self, v: Vector<T>) -> Vector<T> {
        self.transform_vector_impl(v).truncate()
    }

    pub fn transform_point(&self, p: Point<T>) -> Point<T> {
        self.transform_vector(p.to_vector()).to_point()
    }

    pub fn transform_rect(&self, rect: Rect<T>) -> Quad<T> {
        Quad {
            a: self.transform_point(rect.top_left()),
            b: self.transform_point(rect.top_right()),
            c: self.transform_point(rect.bottom_right()),
            d: self.transform_point(rect.bottom_left()),
        }
    }

    fn transform_vector_impl(&self, v: Vector<T>) -> Vector4d<T> {
        self.transform_vector4d(Vector4d {
            dx: v.dx,
            dy: v.dy,
            dz: T::zero(),
            dw: T::one(),
        })
    }

    fn transform_vector4d(&self, v: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            dx: self.m11 * v.dx + self.m21 * v.dy + self.m31 * v.dz + self.m41 * v.dw,
            dy: self.m12 * v.dx + self.m22 * v.dy + self.m32 * v.dz + self.m42 * v.dw,
            dz: self.m13 * v.dx + self.m23 * v.dy + self.m33 * v.dz + self.m43 * v.dw,
            dw: self.m14 * v.dx + self.m24 * v.dy + self.m34 * v.dz + self.m44 * v.dw,
        }
    }
}

impl<T: en::Num> From<Transform<T>> for Transform3d<T> {
    fn from(
        Transform {
            m11,
            m12,
            m21,
            m22,
            m31: m41,
            m32: m42,
        }: Transform<T>,
    ) -> Self {
        Self {
            m11,
            m12,
            m21,
            m22,
            m41,
            m42,
            ..Self::identity()
        }
    }
}

impl<T: en::Num> From<[[T; 4]; 4]> for Transform3d<T> {
    fn from(
        [[m11, m12, m13, m14], [m21, m22, m23, m24], [m31, m32, m33, m34], [m41, m42, m43, m44]]: [[T; 4]; 4],
    ) -> Self {
        Self::row_major(
            m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T, Src, Dst> From<Transform3d<T>> for euclid::Transform3D<T, Src, Dst> {
    fn from(t: Transform3d<T>) -> euclid::Transform3D<T, Src, Dst> {
        Self::new(
            t.m11, t.m12, t.m13, t.m14, t.m21, t.m22, t.m23, t.m24, t.m31, t.m32, t.m33, t.m34,
            t.m41, t.m42, t.m43, t.m44,
        )
    }
}

#[cfg(feature = "euclid")]
impl<T, Src, Dst> From<euclid::Transform3D<T, Src, Dst>> for Transform3d<T> {
    fn from(t: euclid::Transform3D<T, Src, Dst>) -> Transform3d<T> {
        Self {
            m11: t.m11,
            m12: t.m12,
            m13: t.m13,
            m14: t.m14,
            m21: t.m21,
            m22: t.m22,
            m23: t.m23,
            m24: t.m24,
            m31: t.m31,
            m32: t.m32,
            m33: t.m33,
            m34: t.m34,
            m41: t.m41,
            m42: t.m42,
            m43: t.m43,
            m44: t.m44,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn transforming_some_vectors() {
        assert_eq!(
            Transform3d::default().transform_vector(Vector::<f32>::zero()),
            Vector { dx: 0.0, dy: 0.0 }
        );
        assert_eq!(
            Transform3d::default().transform_vector(Vector::new(42.0f32, -12.0)),
            Vector {
                dx: 42.0,
                dy: -12.0,
            }
        );
        assert_eq!(
            Transform3d::from(Transform::from_translation(1.0f32, 2.0))
                .transform_vector(Vector::zero()),
            Vector { dx: 1.0, dy: 2.0 }
        );
        assert_eq!(
            Transform3d::from(Transform::from_scale(1.0, 2.0))
                .transform_vector(Vector::new(3.0f32, 4.0)),
            Vector { dx: 3.0, dy: 8.0 }
        );
        {
            let transformed = Transform3d::from(Transform::from_rotation(
                Angle::from_degrees(90.0),
                Point::zero(),
            ))
            .transform_vector(Vector::new(1.0, 1.0));
            let result = Vector { dx: 1.0, dy: -1.0 };
            // float precision makes these not exactly equal
            assert_approx_eq!(transformed.dx, result.dx);
            assert_approx_eq!(transformed.dy, result.dy);
        }
    }
}
