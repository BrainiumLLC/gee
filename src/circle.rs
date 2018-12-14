use crate::Point;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Circle<T> {
    point:  Point<T>,
    radius: T,
}

impl<T> Circle<T> {
    pub fn new(point: Point<T>, radius: T) -> Self {
        Circle { point, radius }
    }
}
