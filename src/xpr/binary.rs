use std::ops::{Add, Sub};
use super::expression::{Xpr, XprWrapper};
use crate::dual::Dual;

/// Operation type.
#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
}

/// Structure to hold binary expression.
#[derive(Copy, Clone, Debug)]
pub struct BinaryXpr<T: Xpr, U: Xpr> {
    /// 'operation' - operation type.
    operation: Operation,
    /// 'left' - the left part of expression.
    left: T,
    /// 'right' - the right part of expression.
    right: U,
}

impl<T: Xpr, U: Xpr> Xpr for BinaryXpr<T, U> {
    fn value(&self) -> f64 {
        match self.operation {
            Operation::Add => { self.left.value() + self.right.value() }
        }
    }
}

impl<T: Xpr, U: Xpr> BinaryXpr<T, U> {
    fn add_expression(left: T, right: U) -> Self {
        Self {
            operation: Operation::Add,
            left,
            right,
        }
    }
}

impl Add for Dual {
    type Output = XprWrapper<BinaryXpr<Dual, Dual>>;
    fn add(self, other: Dual) -> Self::Output {
        XprWrapper::<BinaryXpr<Dual, Dual>>{xpr: BinaryXpr::<Dual, Dual>::add_expression(self, other)}
    }
}

impl<U: Xpr> Add<XprWrapper<U>> for Dual {
    type Output = XprWrapper<BinaryXpr<Dual, U>>;
    fn add(self, other: XprWrapper<U>) -> Self::Output {
        XprWrapper::<BinaryXpr<Dual, U>>{xpr: BinaryXpr::<Dual, U>::add_expression(self, other.xpr)}
    }
}

impl<T: Xpr, U: Xpr> Add<U> for XprWrapper<T> {
    type Output = XprWrapper<BinaryXpr<T, U>>;
    fn add(self, other: U) -> Self::Output {
        XprWrapper::<BinaryXpr<T, U>>{xpr: BinaryXpr::<T, U>::add_expression(self.xpr, other)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let a = Dual::from(1.0);
        let b = Dual::from(1.0);
        let c = Dual::from(1.0);
        let d = a + b;
        let e = c + d;
        let f = d + c;
    }
}