use super::{assign::Assign, wrapper::XprWrapper};
use crate::dual::Dual;

/// Assign Dual to Dual.
/// Just simple implementation for Dual number.
impl Assign for Dual {
    fn assign(&self, other: &mut Dual) {
        other.val = self.val;
        other.der = self.der;
    }

    fn assign_add(&self, target: &mut Dual) {
        target.val += self.val;
        target.der.set(target.der.get() + self.der.get());
    }

    fn assign_sub(&self, target: &mut Dual) {
        target.val -= self.val;
        target.der.set(target.der.get() - self.der.get());
    }

    fn assign_mul(&self, target: &mut Dual) {
        target.der.set(target.der.get() * self.val + self.der.get() * target.val);
        target.val *= self.val;
    }

    fn assign_div(&self, target: &mut Dual) {
        target.der.set(-(target.der.get() * self.val - self.der.get() * target.val) / self.val / self.val);
        target.val /= self.val;
    }
}

impl<T: Assign + Copy + Clone> From<XprWrapper<T>> for Dual {
    fn from(wxpr: XprWrapper<T>) -> Self {
        let mut val = Dual::new(0.0);
        wxpr.xpr.assign(&mut val);
        val
    }
}
