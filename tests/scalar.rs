extern crate g;
use g::Scalar;

struct U;

fn ord_test<O: Ord>() {}

#[test]
fn ord() {
    let _ = Scalar::<_, U>::new(0i8);
    let _ = Scalar::<_, U>::new(0u32);

    let _ = Scalar::<_, U>::new(-0.0f32);
    let _ = Scalar::<_, U>::new(0.0f32);
    let _ = Scalar::<_, U>::new(1.0f32);
    let _ = Scalar::<_, U>::new(-1.0f32);
    let _ = Scalar::<_, U>::new(std::f32::MAX);
    let _ = Scalar::<_, U>::new(std::f32::MIN);

    let _ = Scalar::<_, U>::new(-0.0f64);
    let _ = Scalar::<_, U>::new(0.0f64);
    let _ = Scalar::<_, U>::new(1.0f64);
    let _ = Scalar::<_, U>::new(-1.0f64);
    let _ = Scalar::<_, U>::new(std::f64::MAX);
    let _ = Scalar::<_, U>::new(std::f64::MIN);

    struct B;
    let _ = Scalar::<_, U>::new(B);

    #[derive(PartialOrd, PartialEq, Ord, Eq)]
    struct B2;
    let _ = Scalar::<_, U>::new(B2);

    ord_test::<Scalar<i8, U>>();
    ord_test::<Scalar<f32, U>>();
    ord_test::<Scalar<f64, U>>();
}

#[test]
#[should_panic]
fn ord_fail() {
    #[derive(PartialOrd, PartialEq)]
    struct B;
    let _ = Scalar::<_, U>::new(B);
}

#[test]
#[should_panic]
fn f32_nan() {
    let _ = Scalar::<_, U>::new(std::f32::NAN);
}

#[test]
#[should_panic]
fn f32_neg_inf() {
    let _ = Scalar::<_, U>::new(std::f32::NEG_INFINITY);
}

#[test]
#[should_panic]
fn f32_pos_inf() {
    let _ = Scalar::<_, U>::new(std::f32::INFINITY);
}

#[test]
#[should_panic]
fn f64_nan() {
    let _ = Scalar::<_, U>::new(std::f64::NAN);
}

#[test]
#[should_panic]
fn f64_neg_inf() {
    let _ = Scalar::<_, U>::new(std::f64::NEG_INFINITY);
}

#[test]
#[should_panic]
fn f64_pos_inf() {
    let _ = Scalar::<_, U>::new(std::f64::INFINITY);
}
