#![deny(rust_2018_idioms, unused_must_use)]

#[macro_use]
mod cast;

mod angle;
mod circle;
mod direction;
mod lerp;
mod line_segment;
mod transform;
mod point;
mod ray;
mod rect;
mod rect_position;
mod size;
mod vector;

pub use self::{
    angle::*, circle::*, direction::*, lerp::*, line_segment::*, transform::*, point::*,
    ray::*, rect::*, rect_position::*, size::*, vector::*,
};
pub use en;
#[cfg(feature = "euclid")]
pub use euclid;
#[cfg(feature = "nalgebra-glm")]
pub use nalgebra_glm as glm;

#[cfg(test)]
pub(crate) mod test {
    pub fn approx_eq(lhs: f32, rhs: f32) -> bool {
        lhs.is_finite() && rhs.is_finite() && ((lhs - 0.00001)..(lhs + 0.00001)).contains(&rhs)
    }

    #[macro_export]
    macro_rules! assert_approx_eq {
        ($lhs:expr, $rhs:expr $(, $t:tt)*) => {{
            let left = $lhs;
            let right = $rhs;
            assert!(
                $crate::test::approx_eq(left, right),
                "approx_eq check failed\n    left: {:?}, right: {:?}",
                left,
                right
            )
        }};
    }
}
