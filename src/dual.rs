
/// Dual number representation.
#[derive(Clone, Copy, Debug)]
pub struct Dual {
    /// Value of Dual number.
    val: f64,
    /// Derivative of Dual number.
    der: f64,
}

impl Dual {
    /// Create new Dual numbed form float number.
    /// 
    /// # Arguments
    /// 
    /// 'val' - value of Dual number.
    pub fn new(val: f64) -> Self {
        Self {
            val,
            der: 0.0,
        }
    }
}

#[test]
fn test_create_new_dual() {
    let x = Dual::new(1.0);
    assert_eq!(x.val, 1.0);
    assert_eq!(x.der, 0.0);
}
