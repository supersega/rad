#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use rad::{Dual, derivative};

#[cfg(test)]
mod tests_value {
    use super::*;

    #[quickcheck]
    fn sum_property(x: Dual, y: Dual) -> bool {
        Dual::from(x + y) == Dual::from(y + x)
    }
}

#[cfg(test)]
mod tests_derivative {
    use super::*;

    #[quickcheck]
    fn constant_property(x: Dual, c: f64) -> bool {
        let constant = |_: Dual| -> Dual { c.into() };
        derivative!(constant(x), x) == 0.0
    }

    #[quickcheck]
    fn self_derivative_property(x: Dual) -> bool {
        let me = |x: Dual| -> Dual { x.into() };
        derivative!(me(x), x) == 1.0
    }

    #[quickcheck]
    fn mul_by_constant_property(x: Dual, c: f64) -> bool {
        let mul_by_c = |x: Dual| -> Dual { (c * x).into() };
        derivative!(mul_by_c(x), x) == c
    }

    #[quickcheck]
    fn sum_property(x: Dual) -> bool {
        let f1 = |x: Dual| -> Dual { (x + x).into() };
        let f2 = |x: Dual| -> Dual { (x * x).into() };
        derivative!(|x: Dual| -> Dual { (f1(x) + f2(x)).into() }(x), x) == derivative!(f1(x), x) + derivative!(f2(x), x)
    }

    #[quickcheck]
    fn mulproperty(x: Dual) -> bool {
        let f1 = |x: Dual| -> Dual { (x + x).into() };
        let f2 = |x: Dual| -> Dual { (x * x).into() };
        derivative!(|x: Dual| -> Dual { (f1(x) * f2(x)).into() }(x), x) == derivative!(f1(x), x) * f2(x).val() + derivative!(f2(x), x) * f1(x).val()
    }
}