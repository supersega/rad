use super::{assign::Assign, expression::{Xpr, XprWrapper}, operation::Op};
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

    fn assign_op(&self, op: Op, other: &mut Dual) {
        match op {
            Op::Add => {
                other.val += self.val;
                other.der += self.der;
            }
            Op::Sub => {
                other.val -= self.val;
                other.der -= self.der;
            }
        }
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

#[test]
fn test_value_from_add_expressions() {
    let a = Dual::from(1.0);
    let b = Dual::from(1.0);
    let c = Dual::from(1.0);
    let d = Dual::from(1.0);
    let e = a + b;
    let f = c + d;
    let g = Dual::from(e + f);
    let h = Dual::from(f + e);
    assert_eq!(g.val, h.val);
}

#[test]
fn test_value_from_sub_expressions() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(3.0);
    let d = Dual::from(4.0);
    let e = a - b;
    let f = d - c;
    let g = Dual::from(e - f);
    let h = Dual::from(f - e);
    let j = Dual::from(a - b - d + c);
    let k = Dual::from(d - c - a + b);
    assert_eq!(j.val, -k.val);
    assert_eq!(g.val, -h.val);
}
}