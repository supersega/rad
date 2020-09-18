use std::ops::Neg;
use super::{assign::Assign, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

/// Unary expression holder.
#[derive(Copy, Clone, Debug)]
pub struct UnXpr<Op> where Op: Xpr + Copy + Clone {
    /// operand of current expression.
    op : Op,
}

/// Negate expression
#[derive(Copy, Clone, Debug)]
pub struct NegXpr<Op: Xpr + Copy + Clone>(UnXpr<Op>);

impl<Op> Xpr for NegXpr<Op> where 
    Op: Xpr + Copy + Clone {
    fn value(&self) -> f64 { - self.0.op.value() }
}

impl<E> Assign for NegXpr<E> where 
    E: Xpr + Copy + Clone + Assign {
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.val = -other.val;
        other.der.set(-other.der.get());
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

macro_rules! impl_un_op(
    ($Op: ident, $op: ident, $Res: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<$Res<Dual>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: $Res(UnXpr{ op: self })}
            }
        }
        
        impl<E: Xpr + Copy + Clone> $Op for XprWrapper<E> {
            type Output = XprWrapper<$Res<E>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: $Res(UnXpr{ op: self.xpr })}
            }
        }
    }
);

impl_un_op!(Neg, neg, NegXpr);

/// Sinus expression
#[derive(Copy, Clone, Debug)]
pub struct SinXpr<Op: Xpr + Copy + Clone>(UnXpr<Op>);

impl<Op> Xpr for SinXpr<Op> where 
    Op: Xpr + Copy + Clone {
    fn value(&self) -> f64 { self.0.op.value().sin() }
}

impl<E> Assign for SinXpr<E> where 
    E: Xpr + Copy + Clone + Assign {
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.val = other.val.sin();
        other.der.set(other.der.get() * other.val.cos());
    }
}

macro_rules! define_and_impl_un_op (
    ($Op: ident, $op: ident, $Res: ident) => {
        /// $Op operation
        pub trait $Op {
            type Output;
            fn $op(self) -> Self::Output;
        }
        impl_un_op!($Op, $op, $Res);
    };
);

define_and_impl_un_op!(Sin, sin, SinXpr);
