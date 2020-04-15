use std::ops::Neg;
use super::{assign::Assign, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

/// Negate expression
#[derive(Copy, Clone, Debug)]
pub struct NegXpr<Arg> where 
    Arg: Xpr + Copy + Clone {
    arg: Arg,
}

impl<Arg> Xpr for NegXpr<Arg> where 
    Arg: Xpr + Copy + Clone {
    fn value(&self) -> f64 { - self.arg.value() }
}

impl<E> Assign for NegXpr<E> where 
    E: Xpr + Copy + Clone + Assign {
    fn assign(&self, other: &mut Dual) {
        self.arg.assign(other);
        other.val = -other.val;
        other.der = -other.der;
    }

    fn assign_add(&self, target: &mut Dual) {
        self.arg.assign_sub(target);
    }

    fn assign_sub(&self, target: &mut Dual) {
        self.arg.assign_add(target);
    }
}

macro_rules! impl_un_op(
    ($Op: ident, $op: ident, $Res: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<$Res<Dual>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: $Res{ arg: self } }
            }
        }
        
        impl<E: Xpr + Copy + Clone> $Op for XprWrapper<E> {
            type Output = XprWrapper<$Res<E>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: $Res{ arg: self.xpr } }
            }
        }
    }
);

impl_un_op!(Neg, neg, NegXpr);

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