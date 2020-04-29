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
}

impl<T: Xpr + Assign> From<XprWrapper<T>> for Dual {
    fn from(wxpr: XprWrapper<T>) -> Self {
        let mut val = Dual::new(0.0);
        wxpr.xpr.assign(&mut val);
        val
    }
}

#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_value_dual_xpr() {
    let dual_num = Dual::from(1.0);
    assert_eq!(dual_num.value(), 1.0);
}
}