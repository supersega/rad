/// Wrap any expression into this 'holder'. This is
/// a workaround for generic operator overloading.
/// All operations in this library must return this
/// wrapper class as a result of of it.
#[derive(Copy, Clone, Debug)]
pub struct XprWrapper<T: Copy + Clone> {
    /// 'xpr' - underlying expression.
    pub(crate) xpr: T
}
