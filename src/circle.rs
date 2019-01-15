use crate::Point;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    point:  Point<T>,
    radius: T,
}

impl<T> Circle<T> {
    pub fn new(point: Point<T>, radius: T) -> Self {
        Circle { point, radius }
    }
}
