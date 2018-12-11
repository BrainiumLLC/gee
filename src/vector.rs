use crate::scalar::Scalar;
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

macro_rules! op_term {
    ($uname: ident, $lname: ident, $uaname: ident, $laname: ident) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS> $uname<Vector<RHS, Unit>>
            for Vector<T, Unit>
        {
            type Output = Vector<Output, Unit>;
            fn $lname(self, rhs: Vector<RHS, Unit>) -> Self::Output {
                Vector::new(self.dx.$lname(rhs.dx), self.dy.$lname(rhs.dy))
            } 
        }

        impl<'a, T: $uname<&'a RHS, Output = Output>, Unit, Output, RHS>
            $uname<&'a Vector<RHS, Unit>> for Vector<T, Unit>
        {
            type Output = Vector<Output, Unit>;
            fn $lname(self, rhs: &'a Vector<RHS, Unit>) -> Self::Output {
                Vector::new(self.dx.$lname(&rhs.dx), self.dy.$lname(&rhs.dy))
            }
        }

        impl<T: $uaname<RHS>, Unit, RHS> $uaname<Vector<RHS, Unit>> for Vector<T, Unit> {
            fn $laname(&mut self, rhs: Vector<RHS, Unit>) {
                self.dx.$laname(rhs.dx);
                self.dy.$laname(rhs.dy)
            }
        }

        impl<'a, T: $uaname<&'a RHS>, Unit, RHS> $uaname<&'a Vector<RHS, Unit>>
            for Vector<T, Unit>
        {
            fn $laname(&mut self, rhs: &'a Vector<RHS, Unit>) {
                self.dx.$laname(&rhs.dx);
                self.dy.$laname(&rhs.dy)
            }
        }
    };
}

macro_rules! op_factor {
    ($uname: ident, $lname: ident, $uaname: ident, $laname: ident) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS: Clone> $uname<RHS> for Vector<T, Unit> {
            type Output = Vector<Output, Unit>;

            fn $lname(self, rhs: RHS) -> Self::Output {
                Vector::new(self.dx.$lname(rhs.clone()), self.dy.$lname(rhs))
            }
        }

        impl<T: $uaname<RHS>, Unit, RHS: Clone> $uaname<RHS> for Vector<T, Unit> {
            fn $laname(&mut self, rhs: RHS) {
                self.dx.$laname(rhs.clone());
                self.dy.$laname(rhs)
            }
        }
    };
}

op_term!(Add, add, AddAssign, add_assign);
op_term!(Sub, sub, SubAssign, sub_assign);
op_factor!(Rem, rem, RemAssign, rem_assign);
op_factor!(Div, div, DivAssign, div_assign);
op_factor!(Mul, mul, MulAssign, mul_assign);
