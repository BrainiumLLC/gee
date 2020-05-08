#![deny(rust_2018_idioms, unused_must_use)]

#[macro_use]
mod cast;

mod angle;
mod circle;
mod direction;
mod lerp;
mod line_segment;
mod mat3x2;
mod mat4;
mod max;
mod min;
mod ordinary_num;
mod point;
mod ray;
mod rect;
mod rect_position;
mod size;
mod vector;
mod vec3;
mod vec4;

#[cfg(feature = "euclid")]
pub use euclid;

pub use self::{
    angle::*, circle::*, direction::*, lerp::*, line_segment::*, mat3x2::*, mat4::*, max::*,
    min::*, ordinary_num::*, point::*, ray::*, rect::*, rect_position::*, size::*, vector::*,
    vec3::*, vec4::*,
};

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
