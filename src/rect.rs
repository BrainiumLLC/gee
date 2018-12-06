use crate::point::Point;
use crate::size::Size;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rect<T, Unit> {
    top_left_point: Point<T, Unit>,
    size: Size<T, Unit>,
}
