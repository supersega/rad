use std::ops::{Add, Sub};
use super::{assign::Assign, expression::{Xpr, XprWrapper}};
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
        let c = a + b;
        let d = b + a;
        assert_eq!(c.xpr.value(), d.xpr.value());
    }
}