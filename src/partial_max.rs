use std::cmp::Ordering;

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> Option<T> {
    Some(match a.partial_cmp(&b)? {
        Ordering::Equal | Ordering::Less => b,
        Ordering::Greater => a,
    })
}
