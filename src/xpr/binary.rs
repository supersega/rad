use std::ops::{Add, Sub};
use super::{assign::Assign, constant::ConstantXpr, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

/// Struct to hold binary expression left and right parts
#[derive(Copy, Clone, Debug)]
pub struct Binary<L, R> where 
    L: Xpr + Copy + Clone, R: Xpr + Copy + Clone {
    /// 'l' - the left part of expression.
    l: L,
    /// 'r' - the right part of expression.
    r: R,
}

/// Binary expression variant.
#[derive(Copy, Clone, Debug)]
pub enum BinXpr<L, R> where 
    L: Xpr + Copy + Clone, R: Xpr + Copy + Clone {
    /// Add expression variant
    Add(Binary<L, R>),
    /// Sub expression variant
    Sub(Binary<L, R>),
}

impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> BinXpr<L, R> {
    /// Left part of expression
    fn l(&self) -> L { match self { Self::Add(xpr) | Self::Sub(xpr) => { xpr.l } } }
}

impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> Xpr for BinXpr<L, R> {
    fn value(&self) -> f64 {
        match self {
            Self::Add(xpr) => { xpr.l.value() + xpr.r.value() }
            Self::Sub(xpr) => { xpr.l.value() - xpr.r.value() }
        }
    }
}

impl<L: Xpr + Copy + Clone + Assign, R: Xpr + Copy + Clone + Assign> Assign for BinXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.l().assign(target);
        match self {
            Self::Add(xpr) => { xpr.r.assign_add(target); }
            Self::Sub(xpr) => { xpr.r.assign_sub(target); }
        }
    }

    fn assign_add(&self, target: &mut Dual) {
        match self {
            Self::Add(xpr) => {
                xpr.l.assign_add(target);
                xpr.r.assign_add(target);
            }
            Self::Sub(xpr) => {
                xpr.l.assign_add(target);
                xpr.r.assign_sub(target);
            }
        }
    }

    fn assign_sub(&self, target: &mut Dual) {
        match self {
            Self::Add(xpr) => {
                xpr.l.assign_sub(target);
                xpr.r.assign_sub(target);
            }
            Self::Sub(xpr) => {
                xpr.l.assign_sub(target);
                xpr.r.assign_add(target);
            }
        }
    }

    // FIXME: assign other default
    // let mut aux = Dual::new(0.0);
    // self.assign(&mut aux);
    // aux.assign_op(op, other);
}

macro_rules! impl_bin_op(
    ($Op: ident, $op: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<BinXpr<Dual, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: BinXpr::<Dual, Dual>::$Op(Binary {l: self, r: other}) }
            }
        }
        
        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for Dual {
            type Output = XprWrapper<BinXpr<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: BinXpr::<Dual, R>::$Op(Binary {l: self, r: other.xpr}) }
            }
        }
        
        impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<R> for XprWrapper<L> {
            type Output = XprWrapper<BinXpr<L, R>>;
            fn $op(self, other: R) -> Self::Output {
                Self::Output{xpr: BinXpr::<L, R>::$Op(Binary {l: self.xpr, r: other}) }
            }
        }
        
        impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<BinXpr<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: BinXpr::<L, R>::$Op(Binary {l: self.xpr, r: other.xpr}) }
            }
        }

        impl $Op<f64> for Dual {
            type Output = XprWrapper<BinXpr<Dual, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: BinXpr::<Dual, ConstantXpr>::$Op(Binary {l: self, r: other.into()}) }
            }
        }

        impl $Op<Dual> for f64 {
            type Output = XprWrapper<BinXpr<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: BinXpr::<ConstantXpr, Dual>::$Op(Binary {l: self.into(), r: other}) }
            }
        }

        impl<L: Xpr + Copy + Clone> $Op<f64> for XprWrapper<L> {
            type Output = XprWrapper<BinXpr<L, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: BinXpr::<L, ConstantXpr>::$Op(Binary {l: self.xpr, r: other.into()}) }
            }
        }

        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for f64 {
            type Output = XprWrapper<BinXpr<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: BinXpr::<ConstantXpr, R>::$Op(Binary {l: self.into(), r: other.xpr}) }
            }
        }
    }
);

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);

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
}