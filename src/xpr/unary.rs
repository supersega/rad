use std::ops::Neg;
use super::{assign::Assign, expression::{Xpr, XprWrapper}, operation::{Op, UnOp}};
use crate::dual::Dual;

/// Structure to hold unary expression.
#[derive(Copy, Clone, Debug)]
pub struct UnXpr<E: Xpr> {
    /// 'op' - operation type.
    op: UnOp,
    /// 'e' - the underlying expression.
    e: E,
}

impl<E: Xpr> Xpr for UnXpr<E> {
    fn value(&self) -> f64 {
        match self.op {
            UnOp::Neg => { -self.e.value() }
        }
    }
}

impl<E: Xpr + Assign> Assign for UnXpr<E> {
    fn assign(&self, other: &mut Dual) {
        self.e.assign(other);
        match self.op {
            UnOp::Neg => { other.val = -other.val; other.der = -other.der; }
        }
    }

    fn assign_op(&self, op: Op, other: &mut Dual) {
        let mut aux = Dual::from(0.0);
        self.assign(&mut aux);
        aux.assign_op(op, other);
    }
}

macro_rules! impl_un_op(
    ($Op: ident, $op: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<UnXpr<Dual>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: UnXpr::<Dual> { op: UnOp::$Op, e: self, } }
            }
        }
        
        impl<E: Xpr> $Op for XprWrapper<E> {
            type Output = XprWrapper<UnXpr<E>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: UnXpr::<E> { op: UnOp::$Op, e: self.xpr } }
            }
        }
    }
);

impl_un_op!(Neg, neg);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neg() {
        let a = Dual::from(1.0);
        let b = -Dual::from(1.0);
        let c = -(a + b);
        let d = b + a;
        assert_eq!(c.xpr.value(), -d.xpr.value());
    }
}