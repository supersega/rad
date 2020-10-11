use super::expression::{Xpr, XprWrapper};
use crate::dual::Dual;

/// Xpr Dual to Dual.
/// Just simple implementation for Dual number.
impl Xpr for Dual {
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
        target
            .der
            .set(target.der.get() * self.val + self.der.get() * target.val);
        target.val *= self.val;
    }

    fn assign_div(&self, target: &mut Dual) {
        target.der.set(
            -(target.der.get() * self.val - self.der.get() * target.val) / self.val / self.val,
        );
        target.val /= self.val;
    }

    fn assign_pow(&self, target: &mut Dual) {
        let pow = target.val().powf(self.val() - 1.0);

        target
            .der
            .set(pow * (self.der() * target.val().ln() * target.val() + self.val() * target.der()));
        target.val = pow * target.val();
    }
}

impl<T: Xpr> From<XprWrapper<T>> for Dual {
    fn from(wxpr: XprWrapper<T>) -> Self {
        let mut val = Dual::new(0.0);
        wxpr.xpr.assign(&mut val);
        val
    }
}

impl Into<XprWrapper<Dual>> for Dual {
    fn into(self) -> XprWrapper<Dual> {
        XprWrapper { xpr: self }
    }
}
