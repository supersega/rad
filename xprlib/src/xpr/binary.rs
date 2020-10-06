use std::ops::{Add, Sub, Mul, Div};
use super::{expression::{Xpr, XprWrapper}, constant::{ConstantXpr, constant}};
use crate::dual::Dual;

/// Structure which represents binary expression
#[derive(Copy, Clone, Debug)]
pub struct BinXpr<L, R>
where  L: Xpr, R: Xpr {
    /// 'l' - the left part of expression.
    pub(crate) l: L,
    /// 'r' - the right part of expression.
    pub(crate) r: R,
}

/// Add expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct AddXpr<L: Xpr, R: Xpr>(BinXpr<L, R>);

/// Implement Xpr trait for AddXpr
impl<L: Xpr, R: Xpr> Xpr for AddXpr<L, R> {
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
pub struct SubXpr<L: Xpr, R: Xpr>(BinXpr<L, R>);

/// Implement Xpr trait for SubXpr
impl<L: Xpr, R: Xpr> Xpr for SubXpr<L, R> {
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
pub struct MulXpr<L: Xpr, R: Xpr>(BinXpr<L, R>);

/// Implement Xpr trait for MulXpr
impl<L: Xpr, R: Xpr> Xpr for MulXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_mul(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        self.0.l.assign_mul(target);
        self.0.r.assign_mul(target);
    }
}

/// Div expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct DivXpr<L: Xpr, R: Xpr>(BinXpr<L, R>);

/// Implement Xpr trait for DivXpr
impl<L: Xpr, R: Xpr> Xpr for DivXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_div(target);
    }

    fn assign_mul(&self, target: &mut Dual) {
        self.0.l.assign_mul(target);
        self.0.r.assign_div(target);
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

        impl<'l> $Op<Dual> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other})}
            }
        }
        
        impl<'r> $Op<&'r Dual> for Dual {
            type Output = XprWrapper<$Res<Dual, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.clone()})}
            }
        }

        impl<'l, 'r> $Op<&'r Dual> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.clone()})}
            }
        }

        impl<R: Xpr> $Op<XprWrapper<R>> for Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.xpr})}
            }
        }
        
        impl<'l, R: Xpr> $Op<XprWrapper<R>> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.xpr})}
            }
        }

        impl<'r, R: Xpr> $Op<&'r XprWrapper<R>> for Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.xpr})}
            }
        }

        impl<'l, 'r, R: Xpr> $Op<&'r XprWrapper<R>> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.xpr})}
            }
        }

        impl<L: Xpr> $Op<Dual> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other})}
            }
        }
        
        impl<'r, L: Xpr> $Op<&'r Dual> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.clone()})}
            }
        }

        impl<'l, L: Xpr> $Op<Dual> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other})}
            }
        }

        impl<'l, 'r, L: Xpr> $Op<&'r Dual> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.clone()})}
            }
        }

        impl<L: Xpr, R: Xpr> $Op<XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl<'r, L: Xpr, R: Xpr> $Op<&'r XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl<'l, L: Xpr, R: Xpr> $Op<XprWrapper<R>> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl<'l, 'r, L: Xpr, R: Xpr> $Op<&'r XprWrapper<R>> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl $Op<f64> for Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: constant(other)})}
            }
        }

        impl<'l> $Op<f64> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: constant(other)})}
            }
        }

        impl<'r> $Op<&'r f64> for Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: constant(*other)})}
            }
        }

        impl $Op<Dual> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(self), r: other})}
            }
        }

        impl<'r> $Op<&'r Dual> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(self), r: other.clone()})}
            }
        }

        impl<'l> $Op<Dual> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(*self), r: other})}
            }
        }

        impl<'l, 'r> $Op<&'r Dual> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(*self), r: other.clone()})}
            }
        }

        impl<'l, 'r> $Op<&'r f64> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: constant(*other)})}
            }
        }

        impl<L: Xpr> $Op<f64> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: constant(other)})} 
            }
        }

        impl<'r, L: Xpr> $Op<&'r f64> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: constant(*other)})} 
            }
        }

        impl<'l, L: Xpr> $Op<f64> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: constant(other)})} 
            }
        }

        impl<'l, 'r, L: Xpr> $Op<&'r f64> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: constant(*other)})} 
            }
        }

        impl<R: Xpr> $Op<XprWrapper<R>> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(self), r: other.xpr})}
            }
        }

        impl<'l, R: Xpr> $Op<XprWrapper<R>> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(*self), r: other.xpr})}
            }
        }

        impl<'r, R: Xpr> $Op<&'r XprWrapper<R>> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(self), r: other.xpr})}
            }
        }

        impl<'l, 'r, R: Xpr> $Op<&'r XprWrapper<R>> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: constant(*self), r: other.xpr})}
            }
        }
    }
);

impl_bin_op!(Add, add, AddXpr);
impl_bin_op!(Sub, sub, SubXpr);
impl_bin_op!(Mul, mul, MulXpr);
impl_bin_op!(Div, div, DivXpr);
