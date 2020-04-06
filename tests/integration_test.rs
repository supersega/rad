use rad::{Dual, Xpr};

#[test]
fn test_eval() {
    let dual_num = Dual::from(1.0);
    let new_dual = dual_num.eval();
    assert_eq!(new_dual, dual_num);
}