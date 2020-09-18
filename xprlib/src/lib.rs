#![feature(fn_traits)]

pub mod dual;
pub mod xpr;

pub use dual::*;
pub use xpr::*;

#[cfg(test)]
pub mod test {
    #[test]
    fn fun() {
        assert_eq!(1 + 1, 2);
    }
}