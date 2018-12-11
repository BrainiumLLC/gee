use crate::{op, scalar::Scalar};
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector<T, Unit> {
    pub dx: Scalar<T, Unit>,
    pub dy: Scalar<T, Unit>,
    unit:   PhantomData<Unit>,
}

impl<T, Unit> Vector<T, Unit> {
    pub fn new(dx: Scalar<T, Unit>, dy: Scalar<T, Unit>) -> Self {
        Vector {
            dx,
            dy,
            unit: PhantomData,
        }
    }
}

impl<T: Neg<Output = Output>, Unit, Output> Neg for Vector<T, Unit> {
    type Output = Vector<Output, Unit>;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.dx, -self.dy)
    }
}

op_term!(Vector, Add, add, AddAssign, add_assign, dx, dy);
op_term!(Vector, Sub, sub, SubAssign, sub_assign, dx, dy);
op_factor!(Vector, Rem, rem, RemAssign, rem_assign, dx, dy);
op_factor!(Vector, Div, div, DivAssign, div_assign, dx, dy);
op_factor!(Vector, Mul, mul, MulAssign, mul_assign, dx, dy);
