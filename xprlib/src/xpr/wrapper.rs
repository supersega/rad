use super::assign::Assign;

/// Wrap any expression into this 'holder'. This is
/// a workaround for generic operator overloading.
/// All operations in this library must return this
/// wrapper class as a result of of it. All expressions
/// in this crate should implement Assign trait.
#[derive(Copy, Clone, Debug)]
pub struct XprWrapper<T: Assign> {
    /// 'xpr' - underlying expression.
    pub(crate) xpr: T
}
