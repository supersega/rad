/// Use this CopyCell to allow eval gradient for immutable duals.
use toolshed::CopyCell;
/// To be able apply property tests
#[cfg(feature = "test-utils")]
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
    /// negate dual value
    pub(crate) fn neagate(&mut self) { 
        self.val = - self.val;
        self.der.set(-self.der.get());
    }
}

impl From<f64> for Dual {
    fn from(val: f64) -> Self {
        Self { 
            val, 
            der: CopyCell::new(0.0),
        }
    }
}

/// Implement Arbitrary trait for Dual to use it in property tests.
#[cfg(feature = "test-utils")]
impl quickcheck::Arbitrary for Dual {
    /// Just create Dual num from f64
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Dual {
        f64::arbitrary(g).into()
    }
}

/// Implement ApproxEq trait for Dual to use it in property tests
#[cfg(feature = "test-utils")]
impl float_cmp::ApproxEq for Dual {
    /// Use 'Margin' from f64
    type Margin = float_cmp::F64Margin;
    /// Check Dual numbers for approximate equal. Compare value and derivative.
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.val.approx_eq(other.val, margin) && self.der.get().approx_eq(other.der.get(), margin)
    }
}
