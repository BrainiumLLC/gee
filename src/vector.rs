use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Vector<T, Unit> {
    dx: T,
    dy: T,
    unit: PhantomData<Unit>,
}
