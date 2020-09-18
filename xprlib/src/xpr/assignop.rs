use std::ops::{AddAssign, SubAssign, MulAssign};
use super::{assign::Assign, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

macro_rules! impl_assign_op(
    ($Op: ident, $op: ident, $fun: ident) => {
        /// $Op operation Dual to Dual number.
        impl $Op for Dual {
            fn $op(&mut self, other: Dual) { other.$fun(self); }
        }
        /// $Op operation XprWrapper to Dual number.
        impl<E: Xpr + Assign> $Op<XprWrapper<E>> for Dual {
            fn $op(&mut self, other: XprWrapper<E>) { other.xpr.$fun(self); }
        }
    }
);

impl_assign_op!(AddAssign, add_assign, assign_add);
impl_assign_op!(SubAssign, sub_assign, assign_sub);
impl_assign_op!(MulAssign, mul_assign, assign_mul);

#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_add_assign_dual() {
    let mut a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a + b);
    a += b;
    assert_eq!(c, a);
}

#[test]
fn test_add_assign_xpr_wrapper() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let mut c = Dual::from(2.0);
    let d = a - b;
    let c1 = Dual::from(c + d);
    c += d;
    assert_eq!(c1, c);
}

#[test]
fn test_sub_assign_dual() {
    let mut a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a - b);
    a -= b;
    assert_eq!(c, a);
}

#[test]
fn test_sub_assign_xpr_wrapper() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let mut c = Dual::from(2.0);
    let d = a - b;
    let c1 = Dual::from(c - d);
    c -= d;
    assert_eq!(c1, c);
}
}