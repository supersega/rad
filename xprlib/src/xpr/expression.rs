use crate::dual::Dual;

/// Trait to assign expression to Dual number. By 
/// default assign_add, assign_sub create temporary
/// variables, for some cases we can optimize that.
pub trait Xpr: Copy + Clone {
    /// Assign expression to Dual number.
    /// 
    /// # Arguments 
    /// 'target' - assign expression into target.
    fn assign(&self, target: &mut Dual);

    /// Assign sub to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    /// 
    /// # Node
    /// Should be overridden if operation
    /// can avoid temporary variables
    fn assign_add(&self, target: &mut Dual) {
        let mut aux: Dual = 0.0.into();
        self.assign(&mut aux);
        aux.assign_add(target);
    }

    /// Assign add operation to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    /// # Node
    /// Should be overridden if operation
    /// can avoid temporary variables
    fn assign_sub(&self, target: &mut Dual) {
        let mut aux: Dual = 0.0.into();
        self.assign(&mut aux);
        aux.assign_sub(target);
    }

    /// Assign mul operation to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    /// # Node
    /// Should be overridden if operation
    /// can avoid temporary variables
    fn assign_mul(&self, target: &mut Dual) {
        let mut aux: Dual = 0.0.into();
        self.assign(&mut aux);
        aux.assign_mul(target);
    }

    /// Assign div operation to Dual number.
    /// 
    /// # Arguments
    /// 'target' - assign expression into target.
    /// # Node
    /// Should be overridden if operation
    /// can avoid temporary variables
    fn assign_div(&self, target: &mut Dual) {
        let mut aux: Dual = 0.0.into();
        self.assign(&mut aux);
        aux.assign_div(target);
    }
}

/// Wrap any expression into this 'holder'. This is
/// a workaround for generic operator overloading.
/// All operations in this library must return this
/// wrapper class as a result of of it. All expressions
/// in this crate should implement Xpr trait.
#[derive(Copy, Clone, Debug)]
pub struct XprWrapper<T: Xpr> {
    /// 'xpr' - underlying expression.
    pub(crate) xpr: T
}