use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size<T> {
    vector: Vector<T>,
}
