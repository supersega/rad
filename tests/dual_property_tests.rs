#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
use float_cmp::{ApproxEq, F64Margin};

use rad::{derivative, Dual};

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
        Dual::from(val * Dual::from(1.0)).approx_eq(val, F64Margin::default())
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
    fn sum_xpr_ref_f64_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let e1 = a + b;
        let e2 = c + a;
        // &XprWrapper + f64
        Dual::from(&e1 + c).approx_eq(Dual::from(b + e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn sum_xpr_ref_f64_ref_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let e1 = a + b;
        let e2 = c + a;
        let c = &c;
        // &XprWrapper + &f64
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
    fn mul_xpr_ref_f64_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let e1 = a * b;
        let e2 = c * a;
        // &XprWrapper * Dual
        Dual::from(&e1 * c).approx_eq(Dual::from(b * e2), (EPSILON, ULP))
    }

    #[quickcheck]
    fn mul_xpr_ref_f64_ref_associative_property(a: Dual, b: Dual, c: f64) -> bool {
        let e1 = a * b;
        let e2 = c * a;
        let c = &c;
        // &XprWrapper * &f64
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

    #[quickcheck]
    fn ordering_sum(a: Dual, b: Dual, c: Dual) -> bool {
        (a + c >= b + c) == (a >= b)
    }

    #[quickcheck]
    fn ordering_mul(a: Dual, b: Dual, c: Dual) -> bool {
        let c = c * c + 1.0;
        (a * c >= b * c) == (a >= b)
    }

    #[quickcheck]
    fn ordering_trans(a: Dual, b: Dual, c: Dual) -> bool {
        (a < b && b < c) == (a < b && a < c && b < c)
    }

    #[quickcheck]
    fn ordering_sum_mul(a: Dual, b: Dual, c: Dual) -> bool {
        let c = c * c + 1.0;
        (Dual::from(a * (c + c)) >= b * (c + c)) == (a >= b)
    }

    #[quickcheck]
    fn neg_assign_add(x: Dual, y: Dual) -> bool {
        x + (-y) == x - y
    }

    #[quickcheck]
    fn neg_assign_sub(x: Dual, y: Dual) -> bool {
        x - (-y) == x + y
    }

    #[quickcheck]
    fn neg_assign_mul(x: Dual, y: Dual) -> bool {
        x * (-y) == - x * y
    }

    #[quickcheck]
    fn neg_assign_div(x: Dual, y: Dual) -> bool {
        let y = y * y + 1.0;
        x / (-y) == - x / y
    }
}

#[cfg(test)]
mod test_math_functions {
    use super::*;

    #[quickcheck]
    fn sin_test(x: Dual) -> bool {
        let sin = |x: Dual| -> Dual { x.sin().into() };
        derivative!(sin(x), x).approx_eq(x.val().cos(), F64Margin::default())
    }

    #[quickcheck]
    fn sin_sum_test(x: Dual, y: Dual) -> bool {
        let sin_sum = |x: Dual, y: Dual| -> Dual { (x + y).sin().into() };
        derivative!(sin_sum(x, y), x).approx_eq(Dual::from(y + x).val().cos(), F64Margin::default())
    }

    #[quickcheck]
    fn cos_test(x: Dual) -> bool {
        let cos = |x: Dual| -> Dual { x.cos().into() };
        derivative!(cos(x), x).approx_eq(-x.val().sin(), F64Margin::default())
    }

    #[quickcheck]
    fn cos_sum_test(x: Dual, y: Dual) -> bool {
        let cos_sum = |x: Dual, y: Dual| -> Dual { (x + y).cos().into() };
        derivative!(cos_sum(x, y), x)
            .approx_eq(-Dual::from(y + x).val().sin(), F64Margin::default())
    }

    #[quickcheck]
    fn sqrt_test(x: Dual) -> bool {
        let sqrt = |x: Dual| -> Dual { x.sqrt().into() };
        derivative!(sqrt(x), x).approx_eq(1.0 / (2.0 * x.val().sqrt()), F64Margin::default())
    }

    #[quickcheck]
    fn sqrt_sum_test(x: Dual, y: Dual) -> bool {
        let sqrt_sum = |x: Dual, y: Dual| -> Dual { (x + y).sqrt().into() };
        derivative!(sqrt_sum(x, y), x).approx_eq(
            1.0 / (2.0 * (x.val() + y.val()).sqrt()),
            F64Margin::default(),
        )
    }

    #[quickcheck]
    fn ln_test(x: Dual) -> bool {
        let nn_ln = |x: Dual| -> Dual { Dual::from(x * x + 1.0).ln().into() };
        derivative!(nn_ln(x), x).approx_eq(
            2.0 * x.val() / (x.val() * x.val() + 1.0),
            F64Margin::default(),
        )
    }

    #[quickcheck]
    fn ln_sum_test(x: Dual) -> bool {
        let nn_ln = |x: Dual| -> Dual { (x * x + 1.0).ln().into() };
        derivative!(nn_ln(x), x).approx_eq(
            2.0 * x.val() / (x.val() * x.val() + 1.0),
            F64Margin::default(),
        )
    }

    #[quickcheck]
    fn exp_test(x: Dual) -> bool {
        let exp = |x: Dual| -> Dual { x.exp().into() };
        derivative!(exp(x), x).approx_eq(x.val().exp(), F64Margin::default())
    }

    #[quickcheck]
    fn exp_sum_test(x: Dual, y: Dual) -> bool {
        let exp_sum = |x: Dual, y: Dual| -> Dual { (x * x + y * y).exp().into() };
        let aux = (x.val() * x.val() + y.val() * y.val()).exp();
        derivative!(exp_sum(x, y), x).approx_eq(2.0 * x.val() * aux, F64Margin::default())
            && derivative!(exp_sum(x, y), y).approx_eq(2.0 * y.val() * aux, F64Margin::default())
    }

    #[quickcheck]
    fn powf_f64_test(x: Dual, deg: f64) -> bool {
        let powf = |x: Dual, deg: f64| -> Dual { x.powf(deg).into() };
        derivative!(powf(x, deg), x).approx_eq(deg * x.val().powf(deg - 1.0), F64Margin::default())
    }

    #[quickcheck]
    fn powf_dual_test(x: Dual, deg: Dual) -> bool {
        let powf = |x: Dual, deg: Dual| -> Dual { x.powf(deg).into() };
        derivative!(powf(x, deg), x).approx_eq(
            deg.val() * x.val().powf(deg.val() - 1.0),
            F64Margin::default(),
        )
    }

    #[quickcheck]
    fn powf_xpr_dual_test(x: Dual, y: Dual, deg: Dual) -> bool {
        let powf = |x: Dual, y: Dual, deg: Dual| -> Dual { (x * y).powf(deg).into() };
        derivative!(powf(x, y, deg), x).approx_eq(
            deg.val() * (x.val() * y.val()).powf(deg.val() - 1.0) * y.val(),
            F64Margin::default(),
        )
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
        derivative!(|x: Dual| -> Dual { (f1(x) + f2(x)).into() }(x), x).approx_eq(
            derivative!(f1(x), x) + derivative!(f2(x), x),
            F64Margin::default(),
        )
    }

    #[quickcheck]
    fn mul_property(x: Dual) -> bool {
        let f1 = |x: Dual| -> Dual { (x + x).into() };
        let f2 = |x: Dual| -> Dual { (x * x).into() };
        derivative!(|x: Dual| -> Dual { (f1(x) * f2(x)).into() }(x), x).approx_eq(
            derivative!(f1(x), x) * f2(x).val() + derivative!(f2(x), x) * f1(x).val(),
            F64Margin::default(),
        )
    }
}

#[cfg(test)]
mod compare_tests {
    use super::*;

    #[quickcheck]
    fn dual_values_are_eq(f: f64) -> bool {
        let x: Dual = f.into();
        let y: Dual = f.into();
        x == y
    }

    #[quickcheck]
    fn dual_value_is_eq_to_xpr(f: f64) -> bool {
        let x: Dual = f.into();
        let y: Dual = f.into();
        Dual::from(y + x) == x + y
    }

    #[quickcheck]
    fn xpr_is_eq_to_dual_value(x: Dual, y: Dual) -> bool {
        y + x == Dual::from(x + y)
    }

    #[quickcheck]
    fn xpr_is_eq_to_xpr(x: Dual, y: Dual) -> bool {
        x * y == x * y
    }

    #[quickcheck]
    fn dauls_are_eq_even_when_ders_are_not_same(val: f64) -> bool {
        let x: Dual = val.into();
        let y = x;
        // we can do it here, but it is workaround
        y.seed();
        x == y
    }
}
