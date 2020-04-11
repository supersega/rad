use std::ops::Add;
use super::assign::Assign;
use super::expression::{Xpr, XprWrapper};
use super::operation::Op;
use crate::dual::Dual;

/// Structure to hold binary expression.
#[derive(Copy, Clone, Debug)]
pub struct BinaryXpr<L: Xpr, R: Xpr> {
    /// 'op' - operation type.
    op: Op,
    /// 'l' - the left part of expression.
    l: L,
    /// 'r' - the right part of expression.
    r: R,
}

impl<L: Xpr, R: Xpr> Xpr for BinaryXpr<L, R> {
    fn value(&self) -> f64 {
        match self.op {
            Op::Add => { self.l.value() + self.r.value() }
        }
    }
}

impl<L: Xpr + Assign, R: Xpr + Assign> Assign for BinaryXpr<L, R> {
    fn assign(&self, other: &mut Dual) {
        self.l.assign(other);
        self.r.assign_op(self.op, other);
    }

    fn assign_op(&self, op: Op, other: &mut Dual) {
        match op {
            Op::Add => {
                match self.op {
                    // c += a + b
                    Op::Add => {
                        self.l.assign_op(self.op, other);
                        self.r.assign_op(self.op, other);
                    }
                }
            }
        }
        // // FIXME: a lot of aux variables
        // let mut aux = Dual::new(0.0);
        // self.assign(&mut aux);
        // aux.assign_op(op, other);
    }
}

macro_rules! impl_bin_op(
    ($Op: ident, $op: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<BinaryXpr<Dual, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: BinaryXpr::<Dual, Dual> { op: Op::$Op, l: self, r: other, } }
            }
        }
        
        impl<R: Xpr> $Op<XprWrapper<R>> for Dual {
            type Output = XprWrapper<BinaryXpr<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: BinaryXpr::<Dual, R> { op: Op::$Op, l: self, r: other.xpr, } }
            }
        }
        
        impl<L: Xpr, R: Xpr> $Op<R> for XprWrapper<L> {
            type Output = XprWrapper<BinaryXpr<L, R>>;
            fn $op(self, other: R) -> Self::Output {
                Self::Output{xpr: BinaryXpr::<L, R> { op: Op::$Op, l: self.xpr, r: other, } }
            }
        }
        
        impl<L: Xpr, R: Xpr> $Op<XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<BinaryXpr<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: BinaryXpr::<L, R> { op: Op::$Op, l: self.xpr, r: other.xpr, } }
            }
        }
    }
);

impl_bin_op!(Add, add);

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