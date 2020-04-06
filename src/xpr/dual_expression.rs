use super::expression::Xpr;
use crate::dual::Dual;

impl Xpr for Dual {
    fn value(&self) -> f64 {
        self.val()
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