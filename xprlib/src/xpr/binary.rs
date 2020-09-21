use std::ops::{Add, Sub, Mul, Div};
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
        self.0.r.assign_mul(target);
    }
}

/// Div expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct DivXpr<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone>(BinXpr<L, R>);

/// Implement Xpr for DivXpr
impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> Xpr for DivXpr<L, R> {
    fn value(&self) -> f64 { self.0.l.value() / self.0.r.value() }
}

/// Implement Assign trait for DivXpr
impl<L: Xpr + Copy + Clone + Assign, R: Xpr + Copy + Clone + Assign> Assign for DivXpr<L, R> {
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

        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.xpr})}
            }
        }
        
        impl<'l, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.xpr})}
            }
        }

        impl<'r, R: Xpr + Copy + Clone> $Op<&'r XprWrapper<R>> for Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.xpr})}
            }
        }

        impl<'l, 'r, R: Xpr + Copy + Clone> $Op<&'r XprWrapper<R>> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.xpr})}
            }
        }

        impl<L: Xpr + Copy + Clone> $Op<Dual> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other})}
            }
        }
        
        impl<'r, L: Xpr + Copy + Clone> $Op<&'r Dual> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.clone()})}
            }
        }

        impl<'l, L: Xpr + Copy + Clone> $Op<Dual> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other})}
            }
        }

        impl<'l, 'r, L: Xpr + Copy + Clone> $Op<&'r Dual> for &'l XprWrapper<L> {
            type Output = XprWrapper<$Res<L, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.clone()})}
            }
        }

        impl<L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl<'r, L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<&'r XprWrapper<R>> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, R>>;
            fn $op(self, other: &XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.xpr})}
            }
        }

        impl<'l, L: Xpr + Copy + Clone, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for &'l XprWrapper<L> {
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

        impl<'l> $Op<f64> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.into()})}
            }
        }

        impl<'r> $Op<&'r f64> for Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self, r: other.clone().into()})}
            }
        }

        impl $Op<Dual> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.into(), r: other})}
            }
        }

        impl<'r> $Op<&'r Dual> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.into(), r: other.clone()})}
            }
        }

        impl<'l> $Op<Dual> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone().into(), r: other})}
            }
        }

        impl<'l, 'r> $Op<&'r Dual> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, Dual>>;
            fn $op(self, other: &Dual) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone().into(), r: other.clone()})}
            }
        }

        impl<'l, 'r> $Op<&'r f64> for &'l Dual {
            type Output = XprWrapper<$Res<Dual, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone(), r: other.clone().into()})}
            }
        }

        impl<L: Xpr + Copy + Clone> $Op<f64> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.into()})} 
            }
        }

        impl<'r, L: Xpr + Copy + Clone> $Op<&'r f64> for XprWrapper<L> {
            type Output = XprWrapper<$Res<L, ConstantXpr>>;
            fn $op(self, other: &f64) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.xpr, r: other.clone().into()})} 
            }
        }

        impl<R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.into(), r: other.xpr})}
            }
        }

        impl<'l, R: Xpr + Copy + Clone> $Op<XprWrapper<R>> for &'l f64 {
            type Output = XprWrapper<$Res<ConstantXpr, R>>;
            fn $op(self, other: XprWrapper<R>) -> Self::Output {
                Self::Output{xpr: $Res(BinXpr{l: self.clone().into(), r: other.xpr})}
            }
        }
    }
);

impl_bin_op!(Add, add, AddXpr);
impl_bin_op!(Sub, sub, SubXpr);
impl_bin_op!(Mul, mul, MulXpr);
impl_bin_op!(Div, div, DivXpr);
