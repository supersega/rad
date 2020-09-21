#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
use float_cmp::{ApproxEq, F64Margin};

use rad::{Dual, derivative};

const EPSILON: f64 = f64::EPSILON * 10000.0;
const ULP: i64 = 5;

#[cfg(test)]
mod tests_value {
    use super::*;

    #[quickcheck]
    fn keeps_value(val: f64) -> bool {
        Dual::from(val).val().approx_eq(val, F64Margin::default())
    }

    #[quickcheck]
    fn unity_property(val: Dual) -> bool {
        Dual::from(val *  Dual::from(1.0)).approx_eq(val, F64Margin::default())
    }

    #[quickcheck]
    fn zero_property(val: Dual) -> bool {
        Dual::from(val + Dual::from(0.0)).approx_eq(val, F64Margin::default())
    }

    #[quickcheck]
    fn negate_property(val: Dual) -> bool {
        Dual::from(0.0).approx_eq((val + (-val)).into(), F64Margin::default())
    }

    #[quickcheck]
    fn sum_property(x: Dual, y: Dual) -> bool {
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_f64_property(x: Dual, y: f64) -> bool {
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_f64_ref_property(x: Dual, y: f64) -> bool {
        let y = &y;
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_f64_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_f64_ref_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_ref_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sum_refs_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x + y).approx_eq(Dual::from(y + x), F64Margin::default())
    }

    #[quickcheck]
    fn sub_property(x: Dual, y: Dual) -> bool {
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_f64_property(x: Dual, y: f64) -> bool {
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_f64_ref_property(x: Dual, y: f64) -> bool {
        let y = &y;
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_f64_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_f64_ref_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_ref_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sub_refs_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn sum_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        Dual::from(a + b + c).approx_eq(Dual::from(b + c + a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_associative_property_4_args(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        // Cover XprWrapper
        Dual::from(a + b + c + d).approx_eq(Dual::from(d + b + c + a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_xpr_ref_associative_property_4_args(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        let s = a + b;
        // Cover &XprWrapper
        Dual::from(&s + c + d).approx_eq(Dual::from(d + c + &s), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_xpr_ref_xpr_ref_associative_property(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        let l = a + b;
        let r = c + d;
        // Cover &XprWrapper && &XprWrapper
        Dual::from(&l + &r).approx_eq(Dual::from(&r + &l), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let c = &c;
        // XprWrapper + &Dual
        Dual::from(a + b + c).approx_eq(Dual::from(b + c + a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_f64_ref_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let c = &c;
        // XprWrapper + &Dual
        Dual::from(a + b + c).approx_eq(Dual::from(b + c + a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_xpr_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let e1 = a + b;
        let e2 = c + a;
        // &XprWrapper * Dual
        Dual::from(&e1 + c).approx_eq(Dual::from(b + e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_xpr_ref_dual_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let e1 = a + b;
        let e2 = c + a;
        let c = &c;
        // &XprWrapper * &Dual
        Dual::from(&e1 + c).approx_eq(Dual::from(b + e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn neg_property(x: Dual, y: Dual) -> bool {
        Dual::from(x - y).approx_eq(Dual::from(-(y - x)), F64Margin::default())
    }

    #[quickcheck]
    fn mul_property(x: Dual, y: Dual) -> bool {
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_f64_property(x: Dual, y: f64) -> bool {
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_f64_ref_property(x: Dual, y: f64) -> bool {
        let y = &y;
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_f64_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_f64_ref_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_ref_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_refs_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        let y = &y;
        Dual::from(x * y).approx_eq(Dual::from(y * x), F64Margin::default())
    }

    #[quickcheck]
    fn div_property(x: Dual, y: Dual) -> bool {
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_f64_property(x: Dual, y: f64) -> bool {
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_f64_ref_property(x: Dual, y: f64) -> bool {
        let y = &y;
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_f64_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_f64_ref_dual_ref_property(x: Dual, y: f64) -> bool {
        let x = &x;
        let y = &y;
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_ref_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn div_refs_property(x: Dual, y: Dual) -> bool {
        let x = &x;
        let y = &y;
        Dual::from((x / y) * (y / x)).approx_eq(Dual::from(1.0), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        Dual::from(a * b * c).approx_eq(Dual::from(b * c * a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_associative_property_4_args(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        Dual::from(a * b * c * d).approx_eq(Dual::from(d * b * c * a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_xpr_ref_associative_property_4_args(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        let p = a * b;
        Dual::from(&p * c * d).approx_eq(Dual::from(c * d * &p), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_xpr_ref_xpr_ref_associative_property(a: Dual, b: Dual, c: Dual, d: Dual) -> bool {
        let l = a * b;
        let r = c * d;
        // Cover &XprWrapper && &XprWrapper
        Dual::from(&l * &r).approx_eq(Dual::from(&r * &l), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let c = &c;
        // XprWrapper * &Dual
        Dual::from(a * b * c).approx_eq(Dual::from(b * c * a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_f64_ref_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let c = &c;
        // XprWrapper * &Dual
        Dual::from(a * b * c).approx_eq(Dual::from(b * c * a), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_xpr_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let e1 = a * b;
        let e2 = c * a;
        // &XprWrapper * Dual
        Dual::from(&e1 * c).approx_eq(Dual::from(b * e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_xpr_ref_dual_ref_associative_property(a: Dual, b: Dual, c: Dual) -> bool {
        let e1 = a * b;
        let e2 = c * a;
        let c = &c;
        // &XprWrapper * Dual
        Dual::from(&e1 * c).approx_eq(Dual::from(b * e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn distributive_mul_due_sum_property(a: Dual, b: Dual, c: Dual) -> bool {
        Dual::from((a + b) * c).approx_eq(Dual::from(a * c + b * c), (EPSILON, ULP))
    }

    #[quickcheck]
    fn inverse_number_property(x: Dual) -> bool {
        Dual::from((1.0 / x) * x).approx_eq(1.0.into(), F64Margin::default())
    }
}

#[cfg(test)]
mod tests_derivative {
    use super::*;

    #[quickcheck]
    fn constant_property(x: Dual, c: f64) -> bool {
        let constant = |_: Dual| -> Dual { c.into() };
        derivative!(constant(x), x).approx_eq(0.0, F64Margin::default())
    }

    #[quickcheck]
    fn self_derivative_property(x: Dual) -> bool {
        let me = |x: Dual| -> Dual { x.into() };
        derivative!(me(x), x).approx_eq(1.0, F64Margin::default())
    }

    #[quickcheck]
    fn mul_by_constant_property(x: Dual, c: f64) -> bool {
        let mul_by_c = |x: Dual| -> Dual { (c * x).into() };
        derivative!(mul_by_c(x), x).approx_eq(c, F64Margin::default())
    }

    #[quickcheck]
    fn sum_property(x: Dual) -> bool {
        let f1 = |x: Dual| -> Dual { (x + x).into() };
        let f2 = |x: Dual| -> Dual { (x * x).into() };
        derivative!(|x: Dual| -> Dual { (f1(x) + f2(x)).into() }(x), x).approx_eq(derivative!(f1(x), x) + derivative!(f2(x), x), F64Margin::default())
    }

    #[quickcheck]
    fn mul_property(x: Dual) -> bool {
        let f1 = |x: Dual| -> Dual { (x + x).into() };
        let f2 = |x: Dual| -> Dual { (x * x).into() };
        derivative!(|x: Dual| -> Dual { (f1(x) * f2(x)).into() }(x), x).approx_eq(derivative!(f1(x), x) * f2(x).val() + derivative!(f2(x), x) * f1(x).val(), F64Margin::default())
    }
}
