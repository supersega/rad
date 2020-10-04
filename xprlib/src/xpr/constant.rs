use super::assign::Assign;
use crate::dual::Dual;

/// Struct to hold constant expression.
#[derive(Copy, Clone, Debug)]
pub struct ConstantXpr {
    /// 'val' - value of constant expression.
    val: f64,
}

/// Assign constant expression.
impl Assign for ConstantXpr {
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
}

/// Create constant expression.
pub(crate) fn constant(val: f64) -> ConstantXpr {ConstantXpr { val }}
