use super::{assign::Assign};
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

    fn assign_mul(&self, target: &mut Dual) {
        target.val *= self.val;
    }
}
