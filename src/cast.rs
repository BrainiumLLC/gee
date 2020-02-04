use num_traits::{NumCast, ToPrimitive};
use std::{any::type_name, fmt::Debug};

pub fn num<T: NumCast, U: Copy + Debug + ToPrimitive>(n: U) -> T {
    T::from(n).unwrap_or_else(|| {
        panic!(
            "cast failed: value {:?} of type `{}` could not be represented by type `{}`",
            n,
            type_name::<U>(),
            type_name::<T>(),
        )
    })
}
