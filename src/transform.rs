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

    pub fn from_skew(theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        let (sin, cos) = theta.sin_cos();
        Self::row_major(T::one(), T::zero(), -sin, cos, T::zero(), T::zero())
    }

    pub fn from_translation(x: T, y: T) -> Self {
        Self::row_major(T::one(), T::zero(), T::zero(), T::one(), x, y)
    }

    pub fn from_translation_vector(translation: Vector<T>) -> Self {
        Self::from_translation(translation.dx, translation.dy)
    }

    pub fn from_decomposition(
        translation: Vector<T>,
        rotation: Angle<T>,
        skew: Angle<T>,
        scale: Vector<T>,
    ) -> Self
    where
        T: en::Float,
    {
        let (s1, c1) = rotation.sin_cos();

        // Ensure skew angle has same meaning if there is a flip
        let k = if scale.dx * scale.dy < T::zero() {
            Angle::from_radians(-skew.radians())
        } else {
            skew
        };

        let (s2, c2) = (rotation + k).sin_cos();

        let m11 = c1 * scale.dx;
        let m12 = s1 * scale.dx;

        let m21 = -s2 * scale.dy;
        let m22 = c2 * scale.dy;

        let m31 = translation.dx;
        let m32 = translation.dy;

        Self::row_major(m11, m12, m21, m22, m31, m32)
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

    pub fn post_skew(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.post_mul(&Self::from_skew(theta))
    }

    pub fn pre_skew(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.pre_mul(&Self::from_skew(theta))
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

    pub fn decompose(&self) -> (Vector<T>, Angle<T>, Angle<T>, Vector<T>)
    where
        T: en::Float,
    {
        let mut y_flip = T::one();
        let translation = Vector::new(self.m31, self.m32);

        let rotation_x = Angle::from_xy(self.m11, self.m12);
        let rotation_y = Angle::from_xy(self.m21, self.m22);
        let mut skew = (rotation_y - rotation_x).normalize();

        // The reflection axis is undefined, assume Y
        // i.e. rotating   0º and flipping X
        //    = rotating 180º and flipping Y
        if skew.radians() < T::zero() {
            skew = Angle::from_radians(skew.radians().abs());
            y_flip = -T::one();
        }

        // 90º XY angle = 0 skew
        skew -= Angle::FRAC_PI_2();

        let scale = Vector::new(
            Vector::new(self.m11, self.m12).magnitude(),
            Vector::new(self.m21, self.m22).magnitude() * y_flip,
        );

        (translation, rotation_x, skew, scale)
    }

    pub fn map<U: en::Num>(self, mut f: impl FnMut(T) -> U) -> Transform<U> {
        Transform::row_major(
            f(self.m11),
            f(self.m12),
            f(self.m21),
            f(self.m22),
            f(self.m31),
            f(self.m32),
        )
    }

    impl_casts_and_cast!(Transform);
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

    #[test]
    fn decompose_translation() {
        let transform = Transform::from_translation(1.0, 2.0);
        let (t, r, k, s) = transform.decompose();
        assert_approx_eq!(t.dx, 1.0);
        assert_approx_eq!(t.dy, 2.0);
        assert_approx_eq!(s.dx, 1.0);
        assert_approx_eq!(s.dy, 1.0);
        assert_approx_eq!(r.radians(), 0.0);
        assert_approx_eq!(k.radians(), 0.0);
    }

    #[test]
    fn decompose_rotation() {
        let angle = Angle::from_degrees(30.0);
        let transform = Transform::from_rotation(angle);
        let (t, r, k, s) = transform.decompose();
        assert_approx_eq!(t.dx, 0.0);
        assert_approx_eq!(t.dy, 0.0);
        assert_approx_eq!(s.dx, 1.0);
        assert_approx_eq!(s.dy, 1.0);
        assert_approx_eq!(r.radians(), angle.radians());
        assert_approx_eq!(k.radians(), 0.0);
    }

    #[test]
    fn decompose_scale() {
        let transform = Transform::from_scale(2.0, 3.0);
        let (t, r, k, s) = transform.decompose();
        assert_approx_eq!(t.dx, 0.0);
        assert_approx_eq!(t.dy, 0.0);
        assert_approx_eq!(s.dx, 2.0);
        assert_approx_eq!(s.dy, 3.0);
        assert_approx_eq!(r.radians(), 0.0);
        assert_approx_eq!(k.radians(), 0.0);
    }

    #[test]
    fn decompose_skew() {
        let angle = Angle::from_degrees(30.0);
        let transform = Transform::from_skew(angle);
        let (t, r, k, s) = transform.decompose();
        assert_approx_eq!(t.dx, 0.0);
        assert_approx_eq!(t.dy, 0.0);
        assert_approx_eq!(s.dx, 1.0);
        assert_approx_eq!(s.dy, 1.0);
        assert_approx_eq!(r.radians(), 0.0);
        assert_approx_eq!(k.radians(), angle.radians());
    }

    #[test]
    fn recompose() {
        let input = Transform::row_major(1.0, 0.5, -0.5, 0.866025, 0.3, 0.6);
        let (t, r, k, s) = input.decompose();
        let output = Transform::from_decomposition(t, r, k, s);
        assert_approx_eq!(input.m11, output.m11);
        assert_approx_eq!(input.m12, output.m12);
        assert_approx_eq!(input.m21, output.m21);
        assert_approx_eq!(input.m22, output.m22);
        assert_approx_eq!(input.m31, output.m31);
        assert_approx_eq!(input.m32, output.m32);
    }

    #[test]
    fn recompose_reflection() {
        let input = Transform::row_major(1.0, 0.5, 0.5, -0.866025, 0.3, 0.6);
        let (t, r, k, s) = input.decompose();
        let output = Transform::from_decomposition(t, r, k, s);
        assert_approx_eq!(input.m11, output.m11);
        assert_approx_eq!(input.m12, output.m12);
        assert_approx_eq!(input.m21, output.m21);
        assert_approx_eq!(input.m22, output.m22);
        assert_approx_eq!(input.m31, output.m31);
        assert_approx_eq!(input.m32, output.m32);
    }
}
