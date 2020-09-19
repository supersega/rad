use rad::{Dual, Xpr, gradient, derivative};

#[test]
fn test_val_dual_xpr() {
    let dual_num = Dual::from(1.0);
    assert_ne!(dual_num.val(), 1.0);
}

#[test]
fn test_value_from_neg_expressions() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a - b);
    let d = Dual::from(-(b - a));

    assert_eq!(c.val(), d.val());
}
#[test]
fn test_sin_xpr(){
    let a = Dual::from(1.0); 
    let sina1 = a.sin();
    let sina2 = Dual::sin(a);
    assert_eq!(sina1.value(), sina2.value());
    let b = Dual::from(2.0);
    let sinab1 = (a + b).sin();
    let sinab2 = rad::expression::XprWrapper::sin(a + b);
    assert_eq!(sinab1.value(), sinab2.value());
}

#[test]
fn test_add() {
    let a = Dual::from(1.0);
    let b = Dual::from(1.0);
    let c = Dual::from(1.0);
    let d = Dual::from(1.0);
    let e = a + b;
    let f = c + d;
    let g = Dual::from(e + f);
    let h = Dual::from(f + e);
    assert_eq!(g.val(), h.val());
    let g = e + f;
    let h = f + e;
    assert_eq!(g.value(), h.value());
}

#[test]
fn test_add_dual_and_f64() {
    let a = Dual::from(1.0);
    let b = a + 5.0;
    let c = 5.0 + a;
    assert_eq!(b.xpr.value(), c.xpr.value());
}

#[test]
fn test_sub() {
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
    assert_eq!(j.val(), -k.val());
    assert_eq!(g.val(), -h.val());
    let g = e - f;
    let h = f - e;
    let j = a - b - d + c;
    let k = d - c - a + b;
    assert_eq!(j.value(), -k.value());
    assert_eq!(g.value(), -h.value());
}

#[test]
fn test_mul()
{
    let a = Dual::from(7.0);
    let b = Dual::from(2.0);
    let c = Dual::from(3.0);
    let d = Dual::from(4.0);

    let e = a * b;
    let f = d * c;
    let g = Dual::from(a * b * e * f);
    let h = Dual::from(f * e * b * a);

    println!("g: {}", g.val());
    println!("h: {}", h.val());

    assert_eq!(g.val(), h.val());
}

#[test]
fn test_div()
{
    let a = Dual::from(7.0);
    let b = Dual::from(2.0);
    let c = Dual::from(3.0);
    let d = Dual::from(4.0);

    let e = a - b;
    let f = d + c;
    let g = e / f;
    let h = f / e;
    let j = Dual::from(g * h);

    assert_eq!(j.val(), 1.0);
}

#[test]
fn test_add_assign_dual() {
    let mut a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a + b);
    a += b;
    assert_eq!(c, a);
}

#[test]
fn test_add_assign_xpr_wrapper() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let mut c = Dual::from(2.0);
    let d = a - b;
    let c1 = Dual::from(c + d);
    c += d;
    assert_eq!(c1, c);
}

#[test]
fn test_sub_assign_dual() {
    let mut a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let c = Dual::from(a - b);
    a -= b;
    assert_eq!(c, a);
}

#[test]
fn test_sub_assign_xpr_wrapper() {
    let a = Dual::from(1.0);
    let b = Dual::from(2.0);
    let mut c = Dual::from(2.0);
    let d = a - b;
    let c1 = Dual::from(c - d);
    c -= d;
    assert_eq!(c1, c);
}

#[test]
fn derivative_test() {
    let x = Dual::from(2.0);
    let f = |x: Dual| -> Dual { (x * x * x).into() };
    let dfdx = derivative!(f(x), x);
    println!("dfdx: {}", dfdx);
}

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
