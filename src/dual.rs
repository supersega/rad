/// Dual number representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dual {
    /// Value of Dual number.
    pub(crate) val: f64,
    /// Derivative of Dual number.
    pub(crate) der: f64,
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

impl From<f64> for Dual {
    fn from(val: f64) -> Self {
        Self { 
            val, 
            der: 0.0,
        }
    }
}

#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_create_new_dual() {
    let x = Dual::new(1.0);
    assert_eq!(x.val, 1.0);
    assert_eq!(x.der, 0.0);
}

#[test]
fn test_from_into_dual() {
    let val = 2.0;
    let dual1: Dual = val.into();
    let dual2 = Dual::from(val);
    assert_eq!(dual1, dual2);
}
}
