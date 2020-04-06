use crate::dual::Dual;
/// Expression will be result of Dual number computation
pub trait Xpr {
    /// Eval this expression.
    fn eval(&self) -> Dual;
}
