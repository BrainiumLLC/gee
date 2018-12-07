use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size<T, Unit> {
    vector: Vector<T, Unit>,
}
