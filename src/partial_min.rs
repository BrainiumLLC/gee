use std::cmp::Ordering;

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> Option<T> {
    Some(match a.partial_cmp(&b)? {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    })
}
