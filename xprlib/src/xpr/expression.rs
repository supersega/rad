/// Wrap any expression into this 'holder'. This is
/// a workaround for generic operator overloading.
#[derive(Copy, Clone, Debug)]
pub struct XprWrapper<T: Copy + Clone> {
    pub(crate) xpr: T
}
