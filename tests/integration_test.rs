use rad::{Dual, Xpr, gradient};

#[test]
fn gradient_test() {
    let x = vec![Dual::from(1.0), Dual::from(2.0), Dual::from(3.0)];
    let f = |x: &Vec<Dual>| -> Dual { (x[0] + x[0] + x[1] + x[2]).into() };
    let v = gradient!(f(&x), x);
    println!("v: {:#?}", v);
}
