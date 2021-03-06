#[macro_use]
mod cast;

mod angle;
mod circle;
mod direction;
mod lerp;
mod line_segment;
mod point;
mod ray;
mod rect;
mod rect_position;
mod size;
mod transform;
mod vector;

pub use self::{
    angle::*, circle::*, direction::*, lerp::*, line_segment::*, point::*, ray::*, rect::*,
    rect_position::*, size::*, transform::*, vector::*,
};
pub use en;

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
