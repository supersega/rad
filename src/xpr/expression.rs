use crate::dual::Dual;

/// Expression will be result of Dual number computation
pub trait Xpr {
    /// Value of expression.
    fn value(&self) -> f64;
}

/// Wrap any expression into this 'holder'. This is
/// a workaround for generic operator overloading.
#[derive(Copy, Clone, Debug)]
pub struct XprWrapper<T: Xpr> {
    pub xpr: T
}
