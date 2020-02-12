use crate::{cast, Max, Min};
use num_traits::{Float, FloatConst, Num, NumCast, NumRef};
use std::fmt::Debug;

pub trait OrdinaryNum: Copy + Debug + Max + Min + Num + NumCast + NumRef {
    fn two() -> Self {
        cast::num(2)
    }

    fn three() -> Self {
        cast::num(3)
    }

    fn four() -> Self {
        cast::num(4)
    }

    fn five() -> Self {
        cast::num(5)
    }

    fn six() -> Self {
        cast::num(6)
    }

    fn seven() -> Self {
        cast::num(7)
    }

    fn halved(self) -> Self {
        self / Self::two()
    }
}

impl<T> OrdinaryNum for T where T: Copy + Debug + Max + Min + Num + NumCast + NumRef {}

pub trait OrdinaryFloat: Float + FloatConst + OrdinaryNum {}
impl<T> OrdinaryFloat for T where T: Float + FloatConst + OrdinaryNum {}
