use super::expression::{UnXpr, Xpr, XprWrapper};
use crate::dual::Dual;

/// Sinus expression
#[derive(Copy, Clone, Debug)]
pub struct SinXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for SinXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.der.set(other.der.get() * other.val.cos());
        other.val = other.val.sin();
    }
}

/// Cosinus expression
#[derive(Copy, Clone, Debug)]
pub struct CosXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for CosXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.der.set(-other.der.get() * other.val.sin());
        other.val = other.val.cos();
    }
}

/// Sqrt expression
#[derive(Copy, Clone, Debug)]
pub struct SqrtXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for SqrtXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.val = other.val.sqrt();
        other.der.set(other.der.get() / (2.0 * other.val));
    }
}

/// Ln expression
#[derive(Copy, Clone, Debug)]
pub struct LnXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for LnXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.der.set(other.der.get() / other.val);
        other.val = other.val.ln();
    }
}

/// Exponent expression
#[derive(Copy, Clone, Debug)]
pub struct ExpXpr<Op>(UnXpr<Op>)
where
    Op: Xpr;

impl<E> Xpr for ExpXpr<E>
where
    E: Xpr,
{
    fn assign(&self, other: &mut Dual) {
        self.0.op.assign(other);
        other.val = other.val.exp();
        other.der.set(other.der.get() * other.val);
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
    un_op_dual!(sqrt, SqrtXpr);
    un_op_dual!(ln, LnXpr);
    un_op_dual!(exp, ExpXpr);
}

macro_rules! un_op_xpr(
    ($op: ident, $Res: ident, $E: ident) => {
        /// $op operation
        pub fn $op(self) -> XprWrapper<$Res<$E>> { XprWrapper{xpr: $Res(UnXpr{ op: self.xpr })}}
    };
);

impl<E: Xpr> XprWrapper<E> {
    un_op_xpr!(sin, SinXpr, E);
    un_op_xpr!(cos, CosXpr, E);
    un_op_xpr!(sqrt, SqrtXpr, E);
    un_op_xpr!(ln, LnXpr, E);
    un_op_xpr!(exp, ExpXpr, E);
}
