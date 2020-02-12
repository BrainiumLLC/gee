use std::cmp::Ordering;

pub trait Min: PartialOrd {
    fn min(self, other: Self) -> Self;
}

impl<T> Min for T
where
    T: PartialOrd,
{
    fn min(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => self,
            Some(Ordering::Greater) => other,
            None => {
                if self != self {
                    other
                } else {
                    self
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Min;

    #[test]
    fn it_works() {
        let three = 3i32;
        let four = 4i32;
        assert_eq!(Min::min(three, four), 3);
        assert_eq!(Min::min(four, three), 3);
        assert_eq!(Min::min(three, three), 3);
    }

    #[test]
    fn pointer_identity() {
        let three = std::rc::Rc::new(3);
        let three_again = std::rc::Rc::new(3);
        assert_eq!(*Min::min(&*three, &*three_again), 3);
        assert!(std::ptr::eq(
            Min::min(&*three, &*three_again),
            std::cmp::Ord::min(&*three, &*three_again)
        ));
        assert!(std::ptr::eq(
            Min::min(&*three_again, &*three),
            std::cmp::Ord::min(&*three_again, &*three)
        ));
    }

    #[test]
    fn f32() {
        let three = 3.0f32;
        let four = 4.0;
        assert_eq!(Min::min(three, four), f32::min(three, four));
        assert_eq!(Min::min(four, three), f32::min(four, three));
        assert_eq!(Min::min(three, three), f32::min(three, three));

        let nan = std::f32::NAN;
        assert_ne!(nan, nan); // we rely on this
        assert_eq!(Min::min(three, nan), f32::min(three, nan));
        assert_eq!(Min::min(nan, three), f32::min(nan, three));
        assert_eq!(Min::min(nan, nan).is_nan(), f32::min(nan, nan).is_nan());

        let inf = std::f32::INFINITY;
        assert_eq!(Min::min(three, inf), f32::min(three, inf));
        assert_eq!(Min::min(inf, three), f32::min(inf, three));

        let neg_inf = std::f32::NEG_INFINITY;
        assert_eq!(Min::min(three, neg_inf), f32::min(three, neg_inf));
        assert_eq!(Min::min(neg_inf, three), f32::min(neg_inf, three));
    }
}
