use super::{
    binary::BinXpr,
    expression::{Xpr, XprWrapper},
};
use crate::dual::Dual;

/// Powf expression structure which holds binary expression.
#[derive(Copy, Clone, Debug)]
pub struct PowfXpr<L: Xpr, R: Xpr>(BinXpr<L, R>);

/// Implement Xpr trait for PowfXpr.
impl<L: Xpr, R: Xpr> Xpr for PowfXpr<L, R> {
    fn assign(&self, target: &mut Dual) {
        self.0.l.assign(target);
        self.0.r.assign_pow(target);
    }
}

impl Dual {
    pub fn powf<R, Deg>(self, deg: Deg) -> XprWrapper<PowfXpr<Dual, R>>
    where
        R: Xpr,
        Deg: Into<XprWrapper<R>>,
    {
        XprWrapper {
            xpr: PowfXpr(BinXpr {
                l: self,
                r: deg.into().xpr,
            }),
        }
    }
}

impl<L: Xpr> XprWrapper<L> {
    pub fn powf<R, Base>(self, deg: Base) -> XprWrapper<PowfXpr<L, R>>
    where
        R: Xpr,
        Base: Into<XprWrapper<R>>,
    {
        XprWrapper {
            xpr: PowfXpr(BinXpr {
                l: self.xpr,
                r: deg.into().xpr,
            }),
        }
    }
}
