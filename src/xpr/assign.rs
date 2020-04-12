use crate::dual::Dual;

/// Trait to assign expression to Dual number.
pub trait Assign {
    /// Assign expression to Dual number.
    /// 
    /// # Arguments 
    /// 'target' - assign expression into target.
    fn assign(&self, target: &mut Dual);

    /// Assign sub to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    fn assign_add(&self, target: &mut Dual);

    /// Assign add operation to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    fn assign_sub(&self, target: &mut Dual);
}
