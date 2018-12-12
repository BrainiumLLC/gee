use super::{Degrees, Radians};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

// TODO should forward all trait functions, not just those w/o defaults

#[derive(Debug)] // TODO don't derive Debug
pub struct Scalar<T, Unit> {
    t:    T,
    unit: PhantomData<Unit>,
}

impl<T, Unit> Scalar<T, Unit> {
    pub fn new(t: T) -> Self {
        Scalar {
            t,
            unit: PhantomData,
        }
    }
}

// TODO new implementation specialized for float-like types that'll assert not NaN/Infinity/...

impl<T: Default, Unit> Default for Scalar<T, Unit> {
    fn default() -> Self {
        T::default().into()
    }
}

impl<T: Hash, Unit> Hash for Scalar<T, Unit> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state)
    }
}

impl<T: Clone, Unit> Clone for Scalar<T, Unit> {
    fn clone(&self) -> Self {
        self.t.clone().into()
    }
}

impl<T: Copy, Unit> Copy for Scalar<T, Unit> {}

impl<T: PartialEq, Unit> Eq for Scalar<T, Unit> {}

impl<T: PartialEq, Unit> PartialEq for Scalar<T, Unit> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}

impl<T: PartialOrd, Unit> PartialOrd for Scalar<T, Unit> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T: PartialOrd, Unit> Ord for Scalar<T, Unit> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl<T, Unit> From<T> for Scalar<T, Unit> {
    fn from(t: T) -> Self {
        Scalar {
            t,
            unit: PhantomData,
        }
    }
}

impl<T, Unit> Borrow<T> for Scalar<T, Unit> {
    fn borrow(&self) -> &T {
        &self.t
    }
}

impl<T: Neg<Output = Output>, Unit, Output> Neg for Scalar<T, Unit> {
    type Output = Scalar<Output, Unit>;
    fn neg(self) -> Self::Output {
        self.t.neg().into()
    }
}

impl<T: Product<A>, Unit, A> Product<Scalar<A, Unit>> for Scalar<T, Unit> {
    fn product<I>(iter: I) -> Scalar<T, Unit>
    where
        I: Iterator<Item = Scalar<A, Unit>>,
    {
        T::product(iter.map(|x| x.t)).into()
    }
}

impl<T: Sum<A>, Unit, A> Sum<Scalar<A, Unit>> for Scalar<T, Unit> {
    fn sum<I>(iter: I) -> Scalar<T, Unit>
    where
        I: Iterator<Item = Scalar<A, Unit>>,
    {
        T::sum(iter.map(|x| x.t)).into()
    }
}

macro_rules! op {
    ($uname: ident, $lname: ident, $uaname: ident, $laname: ident) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS> $uname<Scalar<RHS, Unit>>
            for Scalar<T, Unit>
        {
            type Output = Scalar<Output, Unit>;
            fn $lname(self, rhs: Scalar<RHS, Unit>) -> Self::Output {
                self.t.$lname(rhs.t).into()
            }
        }

        impl<'a, T: $uname<&'a RHS, Output = Output>, Unit, Output, RHS>
            $uname<&'a Scalar<RHS, Unit>> for Scalar<T, Unit>
        {
            type Output = Scalar<Output, Unit>;
            fn $lname(self, rhs: &'a Scalar<RHS, Unit>) -> Self::Output {
                self.t.$lname(&rhs.t).into()
            }
        }

        impl<T: $uaname<RHS>, Unit, RHS> $uaname<Scalar<RHS, Unit>> for Scalar<T, Unit> {
            fn $laname(&mut self, rhs: Scalar<RHS, Unit>) {
                self.t.$laname(rhs.t)
            }
        }

        impl<'a, T: $uaname<&'a RHS>, Unit, RHS> $uaname<&'a Scalar<RHS, Unit>>
            for Scalar<T, Unit>
        {
            fn $laname(&mut self, rhs: &'a Scalar<RHS, Unit>) {
                self.t.$laname(&rhs.t)
            }
        }
    };
}

op!(Sub, sub, SubAssign, sub_assign);
op!(Rem, rem, RemAssign, rem_assign);
op!(Div, div, DivAssign, div_assign);
op!(Mul, mul, MulAssign, mul_assign);
op!(Add, add, AddAssign, add_assign);

macro_rules! underlying_arg_value {
    ($name:ident, [$head:expr $(, $tail:expr)*],) => {
        $head.$name($($tail),*)
    };
    ($name:ident, [], $self:ident, $($args:tt)*) => {
        underlying_arg_value!{$name, [$self.t], $($args)*}
    };
    ($name:ident, [$($e:expr),* $(,)*], $arg:ident: Self, $($args:tt)*) => {
        underlying_arg_value!{$name, [$($e),*, $arg.t], $($args)*}
    };
    ($name:ident, [$($e:expr),* $(,)*], $arg:ident: Scalar<$t:ty, $u:ty>, $($args:tt)*) => {
        underlying_arg_value!{$name, [$($e),*, $arg.t], $($args)*}
    };
    ($name:ident, [$($e:expr),* $(,)*], $arg:ident: $t:ty, $($args:tt)*) => {
        underlying_arg_value!{$name, [$($e),*, $arg], $($args)*}
    };
}

macro_rules! return_helper {
    (Self, $e:expr) => {
        ($e).into()
    };
    (Scalar<$t:ty, $u:ty>, $e:expr) => {
        ($e).into()
    };
    (Option<Self>, $e:expr) => {
        ($e).map(Into::into)
    };
    (Option<Scalar<$t:ty, $u:ty>>, $e:expr) => {
        ($e).map(Into::into)
    };
    ($t:tt, $e:expr) => {
        $e
    };
}

macro_rules! forward_methods {
    () => {};
    ($(#[$m:meta])*[fn $name:ident($($args:tt)*) -> $($t:tt)*]) => {
        $(#[$m])*
        #[inline]
        pub fn $name($($args)*) -> $($t)* {
            return_helper!{$($t)*, underlying_arg_value!{$name, [], $($args)*,}}
        }
    };
    ($($(#[$m:meta])*[fn $name:ident($($args:tt)*) -> $($t:tt)*])*) => {
        $(forward_methods!{$(#[$m])*[fn $name($($args)*) -> $($t)*]})*
    };
}

macro_rules! float_impl {
    ($(#[$m:meta])*$f:ty) => {
        $(#[$m])*
        impl<Unit> Scalar<$f, Unit> {
            forward_methods! {
                [fn floor(self) -> Self]
                [fn ceil(self) -> Self]
                [fn round(self) -> Self]
                [fn trunc(self) -> Self]
                [fn fract(self) -> Self]
                [fn abs(self) -> Self]
                [fn signum(self) -> Self]
                [fn is_sign_negative(self) -> bool]
                [fn is_sign_positive(self) -> bool]
                [fn is_normal(self) -> bool]
                [fn atan2(self, rhs: Self) -> Scalar<$f, Radians>]
                [fn powi(self, rhs: i32) -> Self]
                [fn powf(self, rhs: $f) -> Self]
                [fn hypot(self, rhs: Self) -> Self]
                [fn mul_add(self, a: $f, b: Self) -> Self]
            }
        }

        $(#[$m])*
        impl Scalar<$f, Radians> {
            forward_methods! {
                [fn sin(self) -> $f]
                [fn cos(self) -> $f]
                [fn tan(self) -> $f]
                [fn sin_cos(self) -> ($f, $f)]
            }
        }

        $(#[$m])*
        impl Scalar<$f, Degrees> {
            #[inline]
            pub fn sin(self) -> $f {
                Scalar::<$f, Radians>::from(self).sin()
            }

            #[inline]
            pub fn cos(self) -> $f {
                Scalar::<$f, Radians>::from(self).cos()
            }
            #[inline]
            pub fn tan(self) -> $f {
                Scalar::<$f, Radians>::from(self).tan()
            }

            #[inline]
            pub fn sin_cos(self) -> ($f, $f) {
                Scalar::<$f, Radians>::from(self).sin_cos()
            }
        }

        impl From<Scalar<$f, Radians>> for Scalar<$f, Degrees> {
            #[inline]
            fn from(other: Scalar<$f, Radians>) -> Self {
                other.t.to_degrees().into()
            }
        }

        impl From<Scalar<$f, Degrees>> for Scalar<$f, Radians> {
            #[inline]
            fn from(other: Scalar<$f, Degrees>) -> Self {
                other.t.to_radians().into()
            }
        }
    };
    ($($(#[$m:meta])*$f:ty),* $(,)*) => {
        $(float_impl!{$(#[$m])*$f})*
    };
}

float_impl! {
    /// [f32 documentation](https://doc.rust-lang.org/nightly/std/primitive.f32.html)
    f32,
    /// [f64 documentation](https://doc.rust-lang.org/nightly/std/primitive.f64.html)
    f64,
}

macro_rules! int_impl {
    ($(#[$m:meta])*$i:ty) => {
        $(#[$m])*
        impl<Unit> Scalar<$i, Unit> {
            forward_methods! {
                [fn count_ones(self) -> u32]
                [fn count_zeros(self) -> u32]
                [fn leading_zeros(self) -> u32]
                [fn trailing_zeros(self) -> u32]
                [fn swap_bytes(self) -> Self]
                [fn pow(self, rhs: u32) -> Self]
                [fn rotate_left(self, places: u32) -> Self]
                [fn rotate_right(self, places: u32) -> Self]
                [fn checked_add(self, rhs: Self) -> Option<Self>]
                [fn checked_sub(self, rhs: Self) -> Option<Self>]
                [fn checked_mul(self, rhs: $i) -> Option<Self>]
                [fn checked_div(self, rhs: $i) -> Option<Self>]
                [fn checked_rem(self, rhs: Self) -> Option<Self>]
                [fn checked_shl(self, rhs: u32) -> Option<Self>]
                [fn checked_shr(self, rhs: u32) -> Option<Self>]
                [fn checked_neg(self) -> Option<Self>]
                [fn saturating_add(self, rhs: Self) -> Self]
                [fn saturating_sub(self, rhs: Self) -> Self]
                [fn saturating_mul(self, rhs: $i) -> Self]
                [fn wrapping_add(self, rhs: Self) -> Self]
                [fn wrapping_sub(self, rhs: Self) -> Self]
                [fn wrapping_mul(self, rhs: $i) -> Self]
                [fn wrapping_div(self, rhs: $i) -> Self]
                [fn wrapping_rem(self, rhs: Self) -> Self]
                [fn wrapping_shl(self, rhs: u32) -> Self]
                [fn wrapping_shr(self, rhs: u32) -> Self]
                [fn wrapping_neg(self) -> Self]
            }
        }
    };
    ($($(#[$m:meta])*$i:ty),* $(,)*) => {
        $(int_impl!{$(#[$m])*$i})*
    };
}

macro_rules! signed_int_impl {
    ($(#[$m:meta])*$i:ty) => {
        int_impl!{$(#[$m])*$i}

        $(#[$m])*
        impl<Unit> Scalar<$i, Unit> {
            forward_methods! {
                [fn checked_abs(self) -> Option<Self>]
                [fn wrapping_abs(self) -> Self]
                [fn abs(self) -> Self]
                [fn signum(self) -> Self]
                [fn is_negative(self) -> bool]
                [fn is_positive(self) -> bool]
            }
        }
    };
    ($($(#[$m:meta])*$i:ty),* $(,)*) => {
        $(signed_int_impl!{$(#[$m])*$i})*
    };
}

signed_int_impl! {
    /// [i8 documentation](https://doc.rust-lang.org/nightly/std/primitive.i8.html)
    i8,
    /// [i16 documentation](https://doc.rust-lang.org/nightly/std/primitive.i16.html)
    i16,
    /// [i32 documentation](https://doc.rust-lang.org/nightly/std/primitive.i32.html)
    i32,
    /// [i64 documentation](https://doc.rust-lang.org/nightly/std/primitive.i64.html)
    i64,
    /// [i128 documentation](https://doc.rust-lang.org/nightly/std/primitive.i128.html)
    i128,
    /// [isize documentation](https://doc.rust-lang.org/nightly/std/primitive.isize.html)
    isize,
}

macro_rules! unsigned_int_impl {
    ($(#[$m:meta])*$i:ty) => {
        int_impl!{$(#[$m])*$i}

        $(#[$m])*
        impl<Unit> Scalar<$i, Unit> {
            forward_methods! {
                [fn is_power_of_two(self) -> bool]
                [fn next_power_of_two(self) -> Self]
                [fn checked_next_power_of_two(self) -> Option<Self>]
            }
        }
    };
    ($($(#[$m:meta])*$i:ty),* $(,)*) => {
        $(unsigned_int_impl!{$(#[$m])*$i})*
    };
}

unsigned_int_impl! {
    /// [u8 documentation](https://doc.rust-lang.org/nightly/std/primitive.u8.html)
    u8,
    /// [u16 documentation](https://doc.rust-lang.org/nightly/std/primitive.u16.html)
    u16,
    /// [u32 documentation](https://doc.rust-lang.org/nightly/std/primitive.u32.html)
    u32,
    /// [u64 documentation](https://doc.rust-lang.org/nightly/std/primitive.u64.html)
    u64,
    /// [u128 documentation](https://doc.rust-lang.org/nightly/std/primitive.u128.html)
    u128,
    /// [usize documentation](https://doc.rust-lang.org/nightly/std/primitive.usize.html)
    usize,
}
