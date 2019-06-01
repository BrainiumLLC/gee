use crate::units::{Ratio, TypeDiv, TypeMul, Unknown};
use num_traits::{One, Zero};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[repr(transparent)]
pub struct Quantity<T, Unit = Unknown>(pub T, pub PhantomData<Unit>);

impl<T, Unit> Quantity<T, Unit> {
    pub fn new(value: T) -> Self {
        Quantity(value, PhantomData)
    }

    pub fn to_unknown(self) -> Quantity<T, Unknown> {
        Quantity::new(self.0)
    }

    pub fn cast<U>(self) -> Quantity<T, U> {
        Quantity::new(self.0)
    }
}

impl<T, U> Quantity<T, Ratio<U, U>> {
    pub fn reduce(self) -> T {
        self.0
    }
}

impl<T: One + Div, N, D> Quantity<T, Ratio<N, D>> {
    pub fn inverse(self) -> Quantity<<T as Div>::Output, Ratio<D, N>> {
        Quantity::new(T::one() / self.0)
    }
}

impl<T: Zero, Unit> Zero for Quantity<T, Unit> {
    fn zero() -> Self {
        Self::new(Zero::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: One + PartialEq> One for Quantity<T, Unknown> {
    fn one() -> Self {
        Self::new(One::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<T, Unit> From<T> for Quantity<T, Unit> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Copy, Unit> Copy for Quantity<T, Unit> {}

impl<T: Clone, Unit> Clone for Quantity<T, Unit> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T: Debug, Unit: Debug> Debug for Quantity<T, Unit> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Quantity")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<T: Default, Unit> Default for Quantity<T, Unit> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, Unit, Rhs> PartialEq<Quantity<Rhs, Unit>> for Quantity<T, Unit>
where
    T: PartialEq<Rhs>,
{
    fn eq(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.eq(&rhs.0)
    }

    fn ne(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.ne(&rhs.0)
    }
}

impl<T, Unit, Rhs> PartialOrd<Quantity<Rhs, Unit>> for Quantity<T, Unit>
where
    T: PartialOrd<Rhs>,
{
    fn partial_cmp(&self, rhs: &Quantity<Rhs, Unit>) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }

    fn lt(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.lt(&rhs.0)
    }

    fn le(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.le(&rhs.0)
    }

    fn gt(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.gt(&rhs.0)
    }

    fn ge(&self, rhs: &Quantity<Rhs, Unit>) -> bool {
        self.0.ge(&rhs.0)
    }
}

impl<T: Eq, Unit> Eq for Quantity<T, Unit> {}

impl<T: Ord, Unit> Ord for Quantity<T, Unit> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }

    fn max(self, rhs: Self) -> Self {
        Self::new(self.0.max(rhs.0))
    }

    fn min(self, rhs: Self) -> Self {
        Self::new(self.0.min(rhs.0))
    }
}

impl<T: Hash, Unit> Hash for Quantity<T, Unit> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
        unsafe {
            let ptr = data.as_ptr() as *const T;
            let len = data.len();
            Hash::hash_slice(std::slice::from_raw_parts(ptr, len), state)
        }
    }
}

impl<T, Unit> Add for Quantity<T, Unit>
where
    T: Add,
{
    type Output = Quantity<T::Output, Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.0 + rhs.0)
    }
}

impl<T, Unit> AddAssign for Quantity<T, Unit>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T, Unit> Sub for Quantity<T, Unit>
where
    T: Sub,
{
    type Output = Quantity<T::Output, Unit>;

    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new(self.0 - rhs.0)
    }
}

impl<T, Unit> SubAssign for Quantity<T, Unit>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T, Unit, UnitRhs> Mul<Quantity<T, UnitRhs>> for Quantity<T, Unit>
where
    T: Mul,
    Unit: TypeMul<UnitRhs>,
{
    type Output = Quantity<T::Output, Unit::Output>;

    fn mul(self, rhs: Quantity<T, UnitRhs>) -> Self::Output {
        Quantity::new(self.0 * rhs.0)
    }
}

// impl<T, Unit, UnitRhs> MulAssign<Quantity<T, UnitRhs>> for Quantity<T, UnitRhs>
// where
//     T: MulAssign<Quantity<T, Unit,
// {
//     fn mul_assign(&mut self, rhs: Self) {
//         self.0 *= rhs.0
//     }
// }

// impl<T, N, D> Div<Quantity<T, UnitRhs>> for Quantity<T, Unit>
// where
//     T: Div<T>,
//     Unit: Div<UnitRhs>,
// {
//     type Output = Ratio<T::Output, Unit, UnitRhs>;

//     fn div(self, rhs: Quantity<T, UnitRhs>) -> Self::Output {
//         Quantity::new(self.0 / rhs.0)
//     }
// }

impl<T> DivAssign for Quantity<T, Unknown>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl<T> Add<T> for Quantity<T, Unknown>
where
    T: Add,
{
    type Output = Quantity<T::Output, Unknown>;

    fn add(self, rhs: T) -> Self::Output {
        Quantity::new(self.0 + rhs)
    }
}

impl<T> AddAssign<T> for Quantity<T, Unknown>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs
    }
}

impl<T> Sub<T> for Quantity<T, Unknown>
where
    T: Sub,
{
    type Output = Quantity<T::Output, Unknown>;

    fn sub(self, rhs: T) -> Self::Output {
        Quantity::new(self.0 - rhs)
    }
}

impl<T> SubAssign<T> for Quantity<T, Unknown>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs
    }
}

impl<T, Unit> Mul<T> for Quantity<T, Unit>
where
    T: Mul,
{
    type Output = Quantity<T::Output, Unit>;

    fn mul(self, rhs: T) -> Self::Output {
        Quantity::new(self.0 * rhs)
    }
}

impl<T, Unit> MulAssign<T> for Quantity<T, Unit>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs
    }
}

impl<T, Unit> Div<T> for Quantity<T, Unit>
where
    T: Div,
{
    type Output = Quantity<T::Output, Unknown>;

    fn div(self, rhs: T) -> Self::Output {
        Quantity::new(self.0 / rhs)
    }
}

impl<T, Unit, UnitRhs> Div<Quantity<T, UnitRhs>> for Quantity<T, Unit>
where
    T: Div,
    Unit: TypeDiv<UnitRhs>,
{
    type Output = Quantity<T::Output, Unit::Output>;

    fn div(self, rhs: Quantity<T, UnitRhs>) -> Self::Output {
        Quantity::new(self.0 / rhs.0)
    }
}

impl<T, Unit> DivAssign<T> for Quantity<T, Unit>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs
    }
}
