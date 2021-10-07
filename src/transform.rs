use crate::{Angle, Point, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DecomposedTransform<T = f32> {
    pub translation: Vector<T>,
    pub scale: Vector<T>,
    pub rotation: Angle<T>,
    pub skew: Angle<T>,
}

impl<T: en::Float> Default for DecomposedTransform<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: en::Float> DecomposedTransform<T> {
    pub fn identity() -> Self {
        Self {
            translation: Vector::zero(),
            scale: Vector::one(),
            rotation: Angle::ZERO(),
            skew: Angle::ZERO(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Transform<T = f32> {
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

    /// Creates a multiplicative identity matrix.
    pub fn identity() -> Self {
        let (_0, _1) = (T::zero(), T::one());
        Self::row_major(_1, _0, _0, _1, _0, _0)
    }

    pub fn from_scale(x: T, y: T) -> Self {
        Self {
            m11: x,
            m22: y,
            ..Self::identity()
        }
    }

    pub fn from_scale_vector(scale: Vector<T>) -> Self {
        Self::from_scale(scale.dx, scale.dy)
    }

    pub fn from_rotation(theta: Angle<T>, center: Point<T>) -> Self
    where
        T: en::Float,
    {
        let (sin, cos) = theta.sin_cos();
        let center = center.to_vector();
        Self {
            m11: cos,
            m12: -sin,
            m21: sin,
            m22: cos,
            ..Self::identity()
        }
        .pre_translate_vector(-center)
        .post_translate_vector(center)
    }

    pub fn from_skew(theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        let (sin, cos) = theta.sin_cos();
        Self {
            m21: -sin,
            m22: cos,
            ..Self::identity()
        }
    }

    pub fn from_translation(x: T, y: T) -> Self {
        Self {
            m31: x,
            m32: y,
            ..Self::identity()
        }
    }

    pub fn from_translation_vector(translation: Vector<T>) -> Self {
        Self::from_translation(translation.dx, translation.dy)
    }

    pub fn from_decomposed(
        DecomposedTransform {
            translation,
            scale,
            rotation,
            skew,
        }: DecomposedTransform<T>,
    ) -> Self
    where
        T: en::Float,
    {
        let (s1, c1) = rotation.sin_cos();

        // Ensure skew angle has same meaning if there is a flip
        let k = if scale.dx * scale.dy < T::zero() {
            skew
        } else {
            skew.map_radians(T::neg)
        };

        let (s2, c2) = (rotation + k).sin_cos();

        let m11 = c1 * scale.dx;
        let m12 = -s1 * scale.dx;

        let m21 = s2 * scale.dy;
        let m22 = c2 * scale.dy;

        let m31 = translation.dx;
        let m32 = translation.dy;

        Self::row_major(m11, m12, m21, m22, m31, m32)
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn post_mul(&self, mat: Self) -> Self {
        Self::row_major(
            self.m11 * mat.m11 + self.m12 * mat.m21,
            self.m11 * mat.m12 + self.m12 * mat.m22,
            self.m21 * mat.m11 + self.m22 * mat.m21,
            self.m21 * mat.m12 + self.m22 * mat.m22,
            self.m31 * mat.m11 + self.m32 * mat.m21 + mat.m31,
            self.m31 * mat.m12 + self.m32 * mat.m22 + mat.m32,
        )
    }

    pub fn pre_mul(&self, mat: Self) -> Self {
        mat.post_mul(*self)
    }

    pub fn post_translate(&self, x: T, y: T) -> Self {
        self.post_mul(Self::from_translation(x, y))
    }

    pub fn post_translate_vector(&self, translation: Vector<T>) -> Self {
        self.post_mul(Self::from_translation_vector(translation))
    }

    pub fn pre_translate(&self, x: T, y: T) -> Self {
        self.pre_mul(Self::from_translation(x, y))
    }

    pub fn pre_translate_vector(&self, translation: Vector<T>) -> Self {
        self.pre_mul(Self::from_translation_vector(translation))
    }

    pub fn post_scale(&self, x: T, y: T) -> Self {
        self.post_mul(Self::from_scale(x, y))
    }

    pub fn post_scale_vector(&self, scale: Vector<T>) -> Self {
        self.post_mul(Self::from_scale_vector(scale))
    }

    pub fn pre_scale(&self, x: T, y: T) -> Self {
        self.pre_mul(Self::from_scale(x, y))
    }

    pub fn pre_scale_vector(&self, scale: Vector<T>) -> Self {
        self.pre_mul(Self::from_scale_vector(scale))
    }

    pub fn post_rotate(&self, theta: Angle<T>, center: Point<T>) -> Self
    where
        T: en::Float,
    {
        self.post_mul(Self::from_rotation(theta, center))
    }

    pub fn pre_rotate(&self, theta: Angle<T>, center: Point<T>) -> Self
    where
        T: en::Float,
    {
        self.pre_mul(Self::from_rotation(theta, center))
    }

    pub fn post_skew(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.post_mul(Self::from_skew(theta))
    }

    pub fn pre_skew(&self, theta: Angle<T>) -> Self
    where
        T: en::Float,
    {
        self.pre_mul(Self::from_skew(theta))
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

    pub fn decompose(&self) -> DecomposedTransform<T>
    where
        T: en::Float,
    {
        let row_x = Vector::new(self.m11, self.m12);
        let row_y = Vector::new(self.m21, self.m22);
        let row_z = Vector::new(self.m31, self.m32);
        let rotation_x = row_x.angle();
        let rotation_y = row_y.angle();
        let (skew, y_flip) = {
            let skew = (rotation_y - rotation_x).normalize();
            // The reflection axis is undefined, assume Y
            // i.e. rotating   0º and flipping X
            //    = rotating 180º and flipping Y
            let (skew, y_flip) = if skew.radians() < T::zero() {
                (skew.map_radians(T::abs), T::one())
            } else {
                (skew, -T::one())
            };
            // 90º XY angle = 0 skew
            (skew - Angle::FRAC_PI_2(), y_flip)
        };
        DecomposedTransform {
            translation: row_z,
            scale: Vector::new(row_x.magnitude(), row_y.magnitude() * y_flip),
            rotation: rotation_x,
            skew,
        }
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

#[cfg(feature = "euclid")]
impl<T, Src, Dst> From<Transform<T>> for euclid::Transform2D<T, Src, Dst> {
    fn from(t: Transform<T>) -> euclid::Transform2D<T, Src, Dst> {
        Self::new(t.m11, t.m12, t.m21, t.m22, t.m31, t.m32)
    }
}

#[cfg(feature = "euclid")]
impl<T, Src, Dst> From<euclid::Transform2D<T, Src, Dst>> for Transform<T> {
    fn from(t: euclid::Transform2D<T, Src, Dst>) -> Transform<T> {
        Self {
            m11: t.m11,
            m12: t.m12,
            m21: t.m21,
            m22: t.m22,
            m31: t.m31,
            m32: t.m32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_approx_eq, Direction};

    #[test]
    fn rotation() {
        let v = Direction::East.unit_vector();

        let r_act = v.transform(Transform::from_rotation(
            Angle::from_degrees(90.0),
            Point::zero(),
        ));
        let r_exp = Direction::North.unit_vector();
        assert_approx_eq!(
            r_act.dx,
            r_exp.dx,
            "90º rotation didn't produce expected dx"
        );
        assert_approx_eq!(
            r_act.dy,
            r_exp.dy,
            "90º rotation didn't produce expected dy"
        );

        let r_act = v.transform(Transform::from_rotation(
            Angle::from_degrees(-90.0),
            Point::zero(),
        ));
        let r_exp = Direction::South.unit_vector();
        assert_approx_eq!(
            r_act.dx,
            r_exp.dx,
            "-90º rotation didn't produce expected dx"
        );
        assert_approx_eq!(
            r_act.dy,
            r_exp.dy,
            "-90º rotation didn't produce expected dy"
        );
    }

    macro_rules! check_decomposition {
        ($transform:expr, $expected:expr) => {
            let actual = $transform.decompose();
            assert_approx_eq!(
                actual.translation.dx,
                $expected.translation.dx,
                "decomposed translation dx didn't match input translation dx"
            );
            assert_approx_eq!(
                actual.translation.dy,
                $expected.translation.dy,
                "decomposed translation dy didn't match input translation dy"
            );
            assert_approx_eq!(
                actual.scale.dx,
                $expected.scale.dx,
                "decomposed scale dx didn't match input scale dx"
            );
            assert_approx_eq!(
                actual.scale.dy,
                $expected.scale.dy,
                "decomposed scale dy didn't match input scale dy"
            );
            assert_approx_eq!(
                actual.rotation.degrees(),
                $expected.rotation.degrees(),
                "decomposed angle didn't match input angle"
            );
            assert_approx_eq!(
                actual.skew.degrees(),
                $expected.skew.degrees(),
                "decomposed skew didn't match input skew"
            );
        };
    }

    #[test]
    fn decompose_translation() {
        let translation = Vector::new(1.0, 2.0);
        check_decomposition!(
            Transform::from_translation_vector(translation),
            DecomposedTransform {
                translation,
                ..Default::default()
            }
        );
    }

    #[test]
    fn decompose_rotation() {
        let rotation = Angle::from_degrees(30.0);
        check_decomposition!(
            Transform::from_rotation(rotation, Point::zero()),
            DecomposedTransform {
                rotation,
                ..Default::default()
            }
        );
    }

    #[test]
    fn decompose_scale() {
        let scale = Vector::new(2.0, 3.0);
        check_decomposition!(
            Transform::from_scale_vector(scale),
            DecomposedTransform {
                scale,
                ..Default::default()
            }
        );
    }

    #[test]
    fn decompose_skew() {
        let skew = Angle::from_degrees(30.0);
        check_decomposition!(
            Transform::from_skew(skew),
            DecomposedTransform {
                skew,
                ..Default::default()
            }
        );
    }

    macro_rules! check_recomposition {
        ($expected:expr) => {
            let actual = Transform::from_decomposed($expected.decompose());
            assert_approx_eq!(
                actual.m11,
                $expected.m11,
                "output m11 didn't match input m11"
            );
            assert_approx_eq!(
                actual.m12,
                $expected.m12,
                "output m12 didn't match input m12"
            );
            assert_approx_eq!(
                actual.m21,
                $expected.m21,
                "output m21 didn't match input m21"
            );
            assert_approx_eq!(
                actual.m22,
                $expected.m22,
                "output m22 didn't match input m22"
            );
            assert_approx_eq!(
                actual.m31,
                $expected.m31,
                "output m31 didn't match input m31"
            );
            assert_approx_eq!(
                actual.m32,
                $expected.m32,
                "output m32 didn't match input m32"
            );
        };
    }

    #[test]
    fn recompose() {
        check_recomposition!(Transform::row_major(1.0, 0.5, -0.5, 0.866025, 0.3, 0.6));
    }

    #[test]
    fn recompose_reflection() {
        check_recomposition!(Transform::row_major(1.0, 0.5, 0.5, -0.866025, 0.3, 0.6));
    }
}
