use rad::{Dual, Xpr, gradient};

#[test]
fn gradient_test() {
    let x = vec![Dual::from(1.0), Dual::from(2.0), Dual::from(3.0)];
    let f = |x: &Vec<Dual>| -> Dual { (x[0] + x[0] + x[1] + x[2]).into() };
    let v = gradient!(f(&x), x);
    println!("v: {:#?}", v);
}

#[test]
fn gradient_2args_test() {
    let x = vec![Dual::from(1.0), Dual::from(5.0)];
    let y = vec![Dual::from(3.0), Dual::from(4.0)];
    let f = |x: &Vec<Dual>, y: &Vec<Dual>| -> Dual { (x[0] + 2.0 * x[1] + 3.0 * y[0] + 4.0 * y[1]).into() };
    let v = gradient!(f(&x, &y), x, y);
    println!("v: {:#?}", v);
}