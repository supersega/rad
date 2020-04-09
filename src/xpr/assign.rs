use crate::dual::Dual;
use super::operation::Op;

/// Trait to assign expression to Dual number.
pub trait Assign {
    /// Assign expression to Dual number.
    /// 
    /// # Arguments 
    /// 'target' - assign expression into target.
    fn assign(&self, target: &mut Dual);
    /// Assign operation to Dual number.
    /// 
    /// # Arguments
    /// 'operation' - operation type (+, -, *, /)
    /// 'target' - assign expression into target.
    fn assign_op(&self, op: Op, target: &mut Dual);
}
