use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Vector { dx, dy }
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vector<RHS>> for Vector<T> {
    type Output = Vector<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Vector::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<'a, T: Add<&'a RHS, Output = Output>, RHS, Output> Add<&'a Vector<RHS>> for Vector<T> {
    type Output = Vector<Output>;
    fn add(self, rhs: &'a Vector<RHS>) -> Self::Output {
        Vector::new(self.dx + &rhs.dx, self.dy + &rhs.dy)
    }
}

impl<'a, T, RHS, Output> Add<Vector<RHS>> for &'a Vector<T>
where
    &'a T: Add<RHS, Output = Output>,
{
    type Output = Vector<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Vector::new(&self.dx + rhs.dx, &self.dy + rhs.dy)
    }
}

impl<'a, 'b, T, RHS, Output> Add<&'b Vector<RHS>> for &'a Vector<T>
where
    &'a T: Add<&'b RHS, Output = Output>,
{
    type Output = Vector<Output>;
    fn add(self, rhs: &'b Vector<RHS>) -> Self::Output {
        Vector::new(&self.dx + &rhs.dx, &self.dy + &rhs.dy)
    }
}
