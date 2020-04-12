use std::ops::Neg;
use super::{assign::Assign, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

/// Binary expression variant.
#[derive(Copy, Clone, Debug)]
pub enum UnXpr<E> where 
    E: Xpr + Copy + Clone {
    /// Negate expression variant
    Neg(E),
}

impl<E> UnXpr<E> where 
    E: Xpr + Copy + Clone {
    /// Internal expression of Unary
    fn xpr(&self) -> E { match self { Self::Neg(xpr) => { *xpr } } }
}

impl<E> Xpr for UnXpr<E> where 
    E: Xpr + Copy + Clone {
    fn value(&self) -> f64 {
        match self {
            Self::Neg(e) => { -e.value() }
        }
    }
}

impl<E> Assign for UnXpr<E> where 
    E: Xpr + Copy + Clone + Assign {
    fn assign(&self, other: &mut Dual) {
        self.xpr().assign(other);
        match self {
            Self::Neg(_) => { other.val = -other.val; other.der = -other.der; }
        }
    }

    fn assign_add(&self, target: &mut Dual) {
        match self {
            Self::Neg(xpr) => { xpr.assign_sub(target) }
        }
    }

    fn assign_sub(&self, target: &mut Dual) {
        match self {
            Self::Neg(xpr) => { xpr.assign_add(target) }
        }
    }
}

macro_rules! impl_un_op(
    ($Op: ident, $op: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<UnXpr<Dual>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: UnXpr::<Dual>::$Op(self) }
            }
        }
        
        impl<E: Xpr + Copy + Clone> $Op for XprWrapper<E> {
            type Output = XprWrapper<UnXpr<E>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: UnXpr::<E>::$Op(self.xpr) }
            }
        }
    }
);

impl_un_op!(Neg, neg);

#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_value_from_neg_expressions() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a - b);
    let d = Dual::from(-(b - a));

    assert_eq!(c.val, d.val);
}
}