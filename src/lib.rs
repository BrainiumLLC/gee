#![deny(rust_2018_idioms, unused_must_use)]

mod angle;
mod circle;
mod direction;
mod lerp;
mod mat3x2;
mod mat4;
mod max;
mod min;
mod point;
mod rect;
mod rect_position;
mod size;
mod vec2;
mod vec3;
mod vec4;

#[cfg(feature = "euclid")]
pub use euclid;

pub use self::{
    angle::*, circle::*, direction::*, lerp::*, mat3x2::*, mat4::*, max::*, min::*, point::*,
    rect::*, rect_position::*, size::*, vec2::*, vec3::*, vec4::*,
};

macro_rules! to_f32_f64_impl {
    ($hkt:ident: $t:ty) => {
        impl $hkt<$t> {
            pub fn to_f32(self) -> $hkt<f32> {
                self.map(|v| v as f32)
            }

            pub fn to_f64(self) -> $hkt<f64> {
                self.map(|v| v as f64)
            }
        }
    };
    ($hkt:ident: $($t:ty),* $(,)*) => {
        $(to_f32_f64_impl! {$hkt: $t})*
    };
}

macro_rules! to_f32_f64 {
    ($($hkt:ident),* $(,)*) => {
        $(to_f32_f64_impl! {
        $hkt:
            i8,i16,i32,i64,i128,
            u8,u16,u32,u64,u128,
        })*
    };
}

to_f32_f64! {
    Point,
    Rect,
    Size,
    Vec2,
    Vec3,
    Vec4,
}
