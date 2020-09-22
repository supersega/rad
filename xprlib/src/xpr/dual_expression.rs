use super::{assign::Assign, expression::{Xpr, XprWrapper}};
use crate::dual::Dual;

impl Xpr for Dual {
    fn value(&self) -> f64 {
        self.val
    }
}

/// Assign Dual to Dual (easy, easy)
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

impl<T: Xpr + Assign + Copy + Clone> From<XprWrapper<T>> for Dual {
    fn from(wxpr: XprWrapper<T>) -> Self {
        let mut val = Dual::new(0.0);
        wxpr.xpr.assign(&mut val);
        val
    }
}
