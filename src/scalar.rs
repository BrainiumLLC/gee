use super::BreaksOrd;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    hash::{Hash, Hasher},
    intrinsics,
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
        Self::try_new(t).expect("`Scalar::new` received an unorderable value")
    }

    pub fn try_new(t: T) -> Option<Self> {
        if !t.breaks_ord() {
            Some(unsafe { Self::new_unchecked(t) })
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(t: T) -> Self {
        debug_assert!(
            !t.breaks_ord(),
            "`Scalar::new_unchecked` received an unorderable value"
        );
        Scalar {
            t,
            unit: PhantomData,
        }
    }
}

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
        Some(self.cmp(other))
    }
}

impl<T: PartialOrd, Unit> Ord for Scalar<T, Unit> {
    default fn cmp(&self, other: &Self) -> Ordering {
        match self.t.partial_cmp(&other.t) {
            Some(o) => o,
            None => {
                debug_assert!(
                    false,
                    "`Scalar::partial_cmp` returned `None`. Some `BreaksOrd` impl has a bug"
                );
                unsafe { intrinsics::unreachable() }
            }
        }
    }
}

impl<T: Ord, Unit> Ord for Scalar<T, Unit> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<T, Unit> From<T> for Scalar<T, Unit> {
    fn from(t: T) -> Self {
        Self::new(t)
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
