/// Use this CopyCell to allow eval gradient for immutable duals.
use toolshed::CopyCell;
/// To be able apply property tests
#[cfg(feature = "quickcheck")]
extern crate quickcheck;

/// Dual number representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dual {
    /// Value of Dual number.
    pub(crate) val: f64,
    /// Derivative of Dual number. This field is "mutable",
    /// since user don't have access to this field from
    /// his program.
    pub(crate) der: CopyCell<f64>,
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
            der: CopyCell::new(0.0),
        }
    }

    /// Set derivative to 1.0 value
    pub fn seed(&self) { self.der.set(1.0) }
    /// Set derivative to 0.0 value
    pub fn unseed(&self) { self.der.set(0.0) }
    /// derivative of dual variable
    pub fn der(&self) -> f64 { self.der.get() }
    /// value of dual variable
    pub fn val(&self) -> f64 { self.val }
}

impl From<f64> for Dual {
    fn from(val: f64) -> Self {
        Self { 
            val, 
            der: CopyCell::new(0.0),
        }
    }
}


#[cfg(feature = "quickcheck")]
impl quickcheck::Arbitrary for Dual {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Dual {
        f64::arbitrary(g).into()
    }
}

#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_create_new_dual() {
    let x = Dual::new(1.0);
    assert_eq!(x.val, 1.0);
    assert_eq!(x.der.get(), 0.0);
}

#[test]
fn test_from_into_dual() {
    let val = 2.0;
    let dual1: Dual = val.into();
    let dual2 = Dual::from(val);
    assert_eq!(dual1, dual2);
}
}
