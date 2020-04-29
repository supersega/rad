use super::{assign::Assign, expression::{Xpr}};
use crate::dual::Dual;

/// Struct to hold constant expression left and right parts
#[derive(Copy, Clone, Debug)]
pub struct ConstantXpr {
    val: f64,
}

/// Constant expression from f64
impl From<f64> for ConstantXpr {
    fn from(val: f64) -> Self {Self { val } }
}

/// Implementation for Xpr for Constant
impl Xpr for ConstantXpr {
    fn value(&self) -> f64 {
        self.val
    }
}

/// Assign Dual to Dual (easy, easy)
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
}
