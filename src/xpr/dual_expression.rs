use super::expression::Xpr;
use crate::dual::Dual;

impl Xpr for Dual {
    fn eval(&self) -> Dual {
        self.clone()
    }
}

#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_eval_dual_xpr() {
    let dual_num = Dual::from(1.0);
    let new_dual = dual_num.eval();
    assert_eq!(dual_num, new_dual);
}
}