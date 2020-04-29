// #![allow(non_snake_case)]
// #![allow(unused_parens)]
// use std::ops::Fn;
// use crate::dual::Dual;

// pub struct ScopedWrt<'a> {
//     wrt: &'a Dual
// }

// impl<'a> ScopedWrt<'a> {
//     fn new(wrt: &'a Dual) -> Self { wrt.seed(); Self { wrt } }
// }

// impl<'a> Drop for ScopedWrt<'a> {
//     fn drop(&mut self) {
//         println!("Droped \n");
//         self.wrt.unseed();
//     }
// }

// pub fn wrt<'a>(x: &'a Dual) -> ScopedWrt<'a> { ScopedWrt::new(x) }

// pub fn derivative<'a, F, At>(f: F, wrt: ScopedWrt<'a>, at: At) 
// where F: Fn(At) -> Dual {
//     println!("We are here \n");
// }

// #[cfg(test)]
// mod test {
// use super::*;
// #[test]
// fn test_derivative() {
//     let x: Dual = 1.0.into();
//     let at = (&x);
//     let f = |x: Dual| -> Dual {(x + x + x).into()};
//     derivative(f, wrt(&x), x);
// }
// }
