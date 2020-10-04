use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use super::{assign::Assign, wrapper::XprWrapper};
use crate::dual::Dual;

macro_rules! impl_assign_op(
    ($Op: ident, $op: ident, $fun: ident) => {
        /// $Op operation Dual to Dual number.
        impl $Op for Dual {
            fn $op(&mut self, other: Dual) { other.$fun(self); }
        }
        /// $Op operation XprWrapper to Dual number.
        impl<E: Assign + Copy + Clone> $Op<XprWrapper<E>> for Dual {
            fn $op(&mut self, other: XprWrapper<E>) { other.xpr.$fun(self); }
        }
    }
);

impl_assign_op!(AddAssign, add_assign, assign_add);
impl_assign_op!(SubAssign, sub_assign, assign_sub);
impl_assign_op!(MulAssign, mul_assign, assign_mul);
impl_assign_op!(DivAssign, div_assign, assign_div);
