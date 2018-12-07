use crate::scalar::Scalar;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector<T, Unit> {
    pub dx:   Scalar<T, Unit>,
    pub dy:   Scalar<T, Unit>,
    unit: PhantomData<Unit>,
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
