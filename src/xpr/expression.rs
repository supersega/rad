use crate::dual::Dual;
/// Expression will be result of Dual number computation
pub trait Xpr {
    /// Value of expression.
    fn value(&self) -> f64;
}

/// Wrap expression
pub struct XprWrapper<T: Xpr>(pub T);
