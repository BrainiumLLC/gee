use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Vector<T> {
    pub fn new<U: Into<T>>(dx: U, dy: U) -> Self {
        Vector {
            dx: dx.into(),
            dy: dy.into(),
        }
    }

    pub fn as_ref(&self) -> Vector<&T> {
        Vector {
            dx: &self.dx,
            dy: &self.dy,
        }
    }

    pub fn as_mut(&mut self) -> Vector<&mut T> {
        Vector {
            dx: &mut self.dx,
            dy: &mut self.dy,
        }
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Vector<RHS>> for Vector<T> {
    type Output = Vector<Output>;
    fn add(self, rhs: Vector<RHS>) -> Self::Output {
        Vector::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}
