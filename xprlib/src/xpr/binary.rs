use std::ops::{Add, Sub, Mul};
use super::{assign::Assign, constant::ConstantXpr, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

/// Structure which represents binary expression
#[derive(Copy, Clone, Debug)]
pub struct BinXpr<L, R>
where  L: Xpr + Copy + Clone, R: Xpr + Copy + Clone {
    /// 'l' - the left part of expression.
    l: L,
    /// 'r' - the right part of expression.
    r: R,
}

/// Add expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct AddXpr<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone>(BinXpr<L, R>);

/// Implement Xpr for AddXpr
impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> Xpr for AddXpr<L, R> {
    fn value(&self) -> f64 { self.0.l.value() + self.0.r.value() }
}

/// Implement Assign trait for AddXpr
impl<L: Xpr + Copy + Clone + Assign, R: Xpr + Copy + Clone + Assign> Assign for AddXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_add(target);
    }

    fn assign_add(&self, target: &mut Dual) {
        self.0.l.assign_add(target);
        self.0.r.assign_add(target);
    }

    fn assign_sub(&self, target: &mut Dual) {
        self.0.l.assign_sub(target);
        self.0.r.assign_sub(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        let mut aux: Dual = *target;
        self.0.l.assign_mul(target);
        self.0.r.assign_mul(&mut aux);
        aux.assign_add(target);
    }
}

/// Sub expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct SubXpr<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone>(BinXpr<L, R>);

/// Implement Xpr for SubXpr
impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> Xpr for SubXpr<L, R> {
    fn value(&self) -> f64 { self.0.l.value() - self.0.r.value() }
}

/// Implement Assign trait for SubXpr
impl<L: Xpr + Copy + Clone + Assign, R: Xpr + Copy + Clone + Assign> Assign for SubXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_sub(target);
    }

    fn assign_add(&self, target: &mut Dual) {
        self.0.l.assign_add(target);
        self.0.r.assign_sub(target);
    }

    fn assign_sub(&self, target: &mut Dual) {
        self.0.l.assign_sub(target);
        self.0.r.assign_add(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        let mut aux: Dual = *target;
        self.0.l.assign_mul(target);
        self.0.r.assign_mul(&mut aux);
        aux.assign_sub(target);
    }
}

/// Mul expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct MulXpr<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone>(BinXpr<L, R>);

/// Implement Xpr for MulXpr
impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> Xpr for MulXpr<L, R> {
    fn value(&self) -> f64 { self.0.l.value() * self.0.r.value() }
}

/// Implement Assign trait for MulXpr
impl<L: Xpr + Copy + Clone + Assign, R: Xpr + Copy + Clone + Assign> Assign for MulXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_mul(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        self.0.l.assign_mul(target);
        self.0.l.assign_mul(target);
    }
}

macro_rules! impl_bin_op(
    ($Op: ident, $op: ident, $Res: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<$Res<Dual, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other})}
            }
        }
        
        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.xpr})}
            }
        }
        
        // FIXME: R is just dual
        impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<R> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: R) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other})}
            }
        }
        
        impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl $Op<f64> for Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.into()})}
            }
        }

        impl $Op<Dual> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.into(), r: other})}
            }
        }

        impl<L: Xpr + Copy + Clone> $Op<f64> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.into()})} 
            }
        }

        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.into(), r: other.xpr})}
            }
        }
    }
);

impl_bin_op!(Add, add, AddXpr);
impl_bin_op!(Sub, sub, SubXpr);
impl_bin_op!(Mul, mul, MulXpr);

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_add() {
    let a = Dual::from(1.0);
    let b = Dual::from(1.0);
    let c = Dual::from(1.0);
    let d = Dual::from(1.0);
    let e = a + b;
    let f = c + d;
    let g = Dual::from(e + f);
    let h = Dual::from(f + e);
    assert_eq!(g.val, h.val);
    let g = e + f;
    let h = f + e;
    assert_eq!(g.value(), h.value());
}

#[test]
fn test_add_dual_and_f64() {
    let a = Dual::from(1.0);
    let b = a + 5.0;
    let c = 5.0 + a;
    assert_eq!(b.xpr.value(), c.xpr.value());
}

#[test]
fn test_sub() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(3.0);
    let d = Dual::from(4.0);
    let e = a - b;
    let f = d - c;
    let g = Dual::from(e - f);
    let h = Dual::from(f - e);
    let j = Dual::from(a - b - d + c);
    let k = Dual::from(d - c - a + b);
    assert_eq!(j.val, -k.val);
    assert_eq!(g.val, -h.val);
    let g = e - f;
    let h = f - e;
    let j = a - b - d + c;
    let k = d - c - a + b;
    assert_eq!(j.value(), -k.value());
    assert_eq!(g.value(), -h.value());
}

#[test]
fn test_mul()
{
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(3.0);
    let d = Dual::from(4.0);

    let e = a - b;
    let f = d + c;
    let g = Dual::from(e * f);
    let h = Dual::from(f * e);

    assert_eq!(g.val, h.val);
}
}