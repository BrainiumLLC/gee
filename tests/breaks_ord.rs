#![feature(specialization)]

extern crate g;

use g::BreaksOrd;

macro_rules! min_max_zero_ord {
    ($t:tt) => {
        assert!(!(0 as $t).breaks_ord());
        assert!(!std::$t::MIN.breaks_ord());
        assert!(!std::$t::MAX.breaks_ord());
    };
    ($($t:tt),* $(,)*) => {
        $(min_max_zero_ord!{$t})*
    };
}

#[test]
fn is_orderable() {
    min_max_zero_ord!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

    assert!(std::f32::INFINITY.breaks_ord());
    assert!(std::f32::NEG_INFINITY.breaks_ord());
    assert!(std::f32::NAN.breaks_ord());

    assert!(std::f64::INFINITY.breaks_ord());
    assert!(std::f64::NEG_INFINITY.breaks_ord());
    assert!(std::f64::NAN.breaks_ord());

    struct A;
    assert!(!A.breaks_ord());

    #[derive(PartialOrd, PartialEq, Ord, Eq)]
    struct A2;
    assert!(!A2.breaks_ord());

    #[derive(PartialOrd, PartialEq, Eq)]
    struct A3;
    assert!(A3.breaks_ord());

    #[derive(PartialOrd, PartialEq, Eq)]
    struct A4;
    unsafe impl BreaksOrd for A4 {
        fn breaks_ord(&self) -> bool {
            false
        }
    }
    assert!(!A4.breaks_ord());
}
