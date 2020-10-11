use super::expression::{UnXpr, Xpr, XprWrapper};
use crate::dual::Dual;
use std::ops::Neg;

/// Negate expression
#[derive(Copy, Clone, Debug)]
pub struct NegXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for NegXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.neagate();
    }

    fn assign_add(&self, target: &mut Dual) {
        self.0.op.assign_sub(target);
    }

    fn assign_sub(&self, target: &mut Dual) {
        self.0.op.assign_add(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        self.0.op.assign_mul(target);
        target.neagate();
    }

    fn assign_div(&self, target: &mut Dual) {
        self.0.op.assign_div(target);
        target.neagate();
    }
}

impl Neg for Dual {
    type Output = XprWrapper<NegXpr<Dual>>;
    fn neg(self) -> Self::Output {
        Self::Output {
            xpr: NegXpr(UnXpr { op: self }),
        }
    }
}

impl<E: Xpr> Neg for XprWrapper<E> {
    type Output = XprWrapper<NegXpr<E>>;
    fn neg(self) -> Self::Output {
        Self::Output {
            xpr: NegXpr(UnXpr { op: self.xpr }),
        }
    }
}
