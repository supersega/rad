use super::expression::{Xpr, XprWrapper};
use crate::dual::Dual;

/// Struct to hold constant expression.
#[derive(Copy, Clone, Debug)]
pub struct ConstantXpr {
    /// 'val' - value of constant expression.
    val: f64,
}

/// Xpr constant expression.
impl Xpr for ConstantXpr {
    fn assign(&self, other: &mut Dual) {
        other.val = self.val;
    }

    fn assign_add(&self, target: &mut Dual) {
        target.val += self.val;
    }

    fn assign_sub(&self, target: &mut Dual) {
        target.val -= self.val;
    }

    fn assign_mul(&self, target: &mut Dual) {
        target.val *= self.val;
    }

    fn assign_pow(&self, target: &mut Dual) {
        let pow = target.val.powf(self.val - 1.0);

        target.der.set(pow * self.val * target.der.get());
        target.val = pow * target.val;
    }
}

/// Create constant expression.
pub(crate) fn constant(val: f64) -> ConstantXpr {
    ConstantXpr { val }
}

impl Into<XprWrapper<ConstantXpr>> for f64 {
    fn into(self) -> XprWrapper<ConstantXpr> {
        XprWrapper {
            xpr: constant(self),
        }
    }
}
