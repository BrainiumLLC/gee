#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Vector { dx, dy }
    }
}
