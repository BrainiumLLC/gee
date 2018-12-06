use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Point<T, Unit> {
    vector: Vector<T, Unit>,
}
