use std::marker::PhantomData;

pub trait TypeMul<T> {
    type Output;
}

pub trait TypeDiv<T> {
    type Output;
}

pub enum Unknown {}

impl<T> TypeMul<T> for Unknown {
    type Output = Unknown;
}

impl<T> TypeDiv<T> for Unknown {
    type Output = Unknown;
}

enum Never {}
enum NeverTwo<N, D> {
    _Never(PhantomData<N>, PhantomData<D>, Never),
}

pub struct Ratio<N = Unknown, D = Unknown> {
    _never: NeverTwo<N, D>,
}

#[macro_export]
macro_rules! def_unit {
    (pub $name:ident) => {
        pub enum $name {}

        impl $crate::units::TypeMul<$crate::units::Unknown> for $name {
            type Output = $crate::units::Unknown;
        }

        impl<T> $crate::units::TypeDiv<T> for $name {
            type Output = $crate::units::Ratio<$name, T>;
        }

        impl<N> $crate::units::TypeMul<$crate::units::Ratio<N, $name>> for $name {
            type Output = N;
        }

        impl<N> $crate::units::TypeMul<$name> for $crate::units::Ratio<N, $name> {
            type Output = N;
        }

        impl<N, D> $crate::units::TypeMul<$crate::units::Ratio<N, $name>>
            for $crate::units::Ratio<$name, D>
        {
            type Output = $crate::units::Ratio<N, D>;
        }

        impl<N, D> $crate::units::TypeDiv<$crate::units::Ratio<D, $name>>
            for $crate::units::Ratio<N, $name>
        {
            type Output = $crate::units::Ratio<N, D>;
        }
    };
}
