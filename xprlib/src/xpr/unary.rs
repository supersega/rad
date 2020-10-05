use std::ops::Neg;
use super::{assign::Assign, wrapper::XprWrapper};
use crate::dual::Dual;

/// Unary expression holder.
#[derive(Copy, Clone, Debug)]
pub struct UnXpr<Op> 
where Op: Assign {
    /// operand of current expression.
    op : Op,
}

/// Negate expression
#[derive(Copy, Clone, Debug)]
pub struct NegXpr<Op: Assign>(UnXpr<Op>);

impl<E> Assign for NegXpr<E> where 
    E: Assign + Assign {
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

macro_rules! impl_un_op(
    ($Op: ident, $op: ident, $Res: ident) => {
        impl $Op for Dual {
            type Output = XprWrapper<$Res<Dual>>;
            fn $op(self) -> Self::Output {
                Self::Output{xpr: $Res(UnXpr{ op: self })}
            }
        }
        
        impl<E: Assign> $Op for XprWrapper<E> {
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
pub struct SinXpr<Op: Assign>(UnXpr<Op>);

impl<E> Assign for SinXpr<E> where 
    E: Assign + Assign {
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.der.set(other.der.get() * other.val.cos());
        other.val = other.val.sin();
    }
}

/// Cosinus expression
#[derive(Copy, Clone, Debug)]
pub struct CosXpr<Op: Assign>(UnXpr<Op>);

impl<E> Assign for CosXpr<E> where 
    E: Assign + Assign {
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.der.set( - other.der.get() * other.val.sin());
        other.val = other.val.cos();
    }
}

/// Sqrt expression
#[derive(Copy, Clone, Debug)]
pub struct SqrtXpr<Op: Assign>(UnXpr<Op>);

impl<E> Assign for SqrtXpr<E> where 
    E: Assign + Assign {
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.val = other.val.sqrt();
        other.der.set( other.der.get() / other.val );
    }
}

macro_rules! un_op_dual(
    ($op: ident, $Res: ident) => {
        /// $op operation
        pub fn $op(self) -> XprWrapper<$Res<Dual>> { XprWrapper{xpr: $Res(UnXpr{ op: self })}}
    };
);

impl Dual {
    un_op_dual!(sin, SinXpr);
    un_op_dual!(cos, CosXpr);
}

macro_rules! un_op_xpr(
    ($op: ident, $Res: ident, $E: ident) => {
        /// $op operation
        pub fn $op(self) -> XprWrapper<$Res<$E>> { XprWrapper{xpr: $Res(UnXpr{ op: self.xpr })}}
    };
);

impl<E: Assign> XprWrapper<E> {
    un_op_xpr!(sin, SinXpr, E);
    un_op_xpr!(cos, CosXpr, E);
    un_op_xpr!(sqrt, SqrtXpr, E);
}
