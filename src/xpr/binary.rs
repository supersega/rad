use std::ops::Add;
use super::assign::Assign;
use super::expression::{Xpr, XprWrapper};
use super::operation::Op;
use crate::dual::Dual;

/// Structure to hold binary expression.
#[derive(Copy, Clone, Debug)]
pub struct BinaryXpr<L: Xpr, R: Xpr> {
    /// 'operation' - operation type.
    op: Op,
    /// 'l' - the left part of expression.
    left: L,
    /// 'r' - the right part of expression.
    right: R,
}

impl<L: Xpr, R: Xpr> Xpr for BinaryXpr<L, R> {
    fn value(&self) -> f64 {
        match self.op {
            Op::Add => { self.left.value() + self.right.value() }
        }
    }
}

impl<L: Xpr + Assign, R: Xpr + Assign> Assign for BinaryXpr<L, R> {
    fn assign(&self, other: &mut Dual) {
        self.left.assign(other);
        self.right.assign_op(self.op, other);
    }

    fn assign_op(&self, op: Op, other: &mut Dual) {
        // FIXME: a lot of aux variables
        let mut aux = Dual::new(0.0);
        self.assign(&mut aux);
        aux.assign_op(op, other);
    }
}

fn add_expression<L: Xpr, R: Xpr>(left: L, right: R) -> BinaryXpr<L, R> {
    BinaryXpr::<L, R> {
        op: Op::Add,
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

impl<R: Xpr> Add<XprWrapper<R>> for Dual {
    type Output = XprWrapper<BinaryXpr<Dual, R>>;
    fn add(self, other: XprWrapper<R>) -> Self::Output {
        Self::Output{xpr: add_expression(self, other.xpr)}
    }
}

impl<L: Xpr, R: Xpr> Add<R> for XprWrapper<L> {
    type Output = XprWrapper<BinaryXpr<L, R>>;
    fn add(self, other: R) -> Self::Output {
        Self::Output{xpr: add_expression(self.xpr, other)}
    }
}

impl<L: Xpr, R: Xpr> Add<XprWrapper<R>> for XprWrapper<L> {
    type Output = XprWrapper<BinaryXpr<L, R>>;
    fn add(self, other: XprWrapper<R>) -> Self::Output {
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