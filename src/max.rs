use std::cmp::Ordering;

pub trait Max: PartialOrd {
    fn max(self, other: Self) -> Self;
}

impl<T> Max for T
where
    T: PartialOrd,
{
    fn max(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) => other,
            Some(Ordering::Greater) | Some(Ordering::Equal) => self,
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
    use super::Max;

    #[test]
    fn it_works() {
        let three = 3i32;
        let four = 4i32;
        assert_eq!(Max::max(three, four), 4);
        assert_eq!(Max::max(four, three), 4);
        assert_eq!(Max::max(three, three), 3);
    }

    #[test]
    fn pointer_identity() {
        let three = std::rc::Rc::new(3);
        let three_again = std::rc::Rc::new(3);
        assert_eq!(*Max::max(&*three, &*three_again), 3);
        assert!(std::ptr::eq(
            Max::max(&*three, &*three_again),
            std::cmp::Ord::max(&*three, &*three_again)
        ));
        assert!(std::ptr::eq(
            Max::max(&*three_again, &*three),
            std::cmp::Ord::max(&*three_again, &*three)
        ));
    }

    #[test]
    fn f32() {
        let three = 3.0f32;
        let four = 4.0;
        assert_eq!(Max::max(three, four), f32::max(three, four));
        assert_eq!(Max::max(four, three), f32::max(four, three));
        assert_eq!(Max::max(three, three), f32::max(three, three));

        let nan = std::f32::NAN;
        assert_ne!(nan, nan); // we rely on this
        assert_eq!(Max::max(three, nan), f32::max(three, nan));
        assert_eq!(Max::max(nan, three), f32::max(nan, three));
        assert_eq!(Max::max(nan, nan).is_nan(), f32::max(nan, nan).is_nan());

        let inf = std::f32::INFINITY;
        assert_eq!(Max::max(three, inf), f32::max(three, inf));
        assert_eq!(Max::max(inf, three), f32::max(inf, three));

        let neg_inf = std::f32::NEG_INFINITY;
        assert_eq!(Max::max(three, neg_inf), f32::max(three, neg_inf));
        assert_eq!(Max::max(neg_inf, three), f32::max(neg_inf, three));
    }
}
