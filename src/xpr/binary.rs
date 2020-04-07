use std::ops::Add;
use super::assign::Assign;
use super::expression::{Xpr, XprWrapper};
use super::operation::Operation;
use crate::dual::Dual;

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

impl<L: Xpr + Assign, R: Xpr + Assign> Assign for BinaryXpr<L, R> {
    fn assign(&self, other: &mut Dual) {
        self.left.assign(other);
        self.right.assign_op(self.operation, other);
    }

    fn assign_op(&self, operation: Operation, other: &mut Dual) {
        // FIXME: a lot of aux variables
        let mut aux = Dual::new(0.0);
        self.assign(&mut aux);
        aux.assign_op(operation, other);
    }
}

fn add_expression<L: Xpr, R: Xpr>(left: L, right: R) -> BinaryXpr<L, R> {
    BinaryXpr::<L, R> {
        operation: Operation::Add,
        left,
        right,
    }
}

impl Add for Dual {
    type Output = XprWrapper<BinaryXpr<Dual, Dual>>;
    fn add(self, other: Dual) -> Self::Output {
        Self::Output{xpr: add_expression(self, other)}
    }
}

impl<U: Xpr> Add<XprWrapper<U>> for Dual {
    type Output = XprWrapper<BinaryXpr<Dual, U>>;
    fn add(self, other: XprWrapper<U>) -> Self::Output {
        Self::Output{xpr: add_expression(self, other.xpr)}
    }
}

impl<T: Xpr, U: Xpr> Add<U> for XprWrapper<T> {
    type Output = XprWrapper<BinaryXpr<T, U>>;
    fn add(self, other: U) -> Self::Output {
        Self::Output{xpr: add_expression(self.xpr, other)}
    }
}

impl<T: Xpr, U: Xpr> Add<XprWrapper<U>> for XprWrapper<T> {
    type Output = XprWrapper<BinaryXpr<T, U>>;
    fn add(self, other: XprWrapper<U>) -> Self::Output {
        Self::Output{xpr: add_expression(self.xpr, other.xpr)}
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
        assert_eq!(e.xpr.value(), e.xpr.value());
    }
}