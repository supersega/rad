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
        let mut u: Dual = 0.0.into();
        self.0.l.assign(&mut u);
        let mut g: Dual = 0.0.into();
        self.0.r.assign(&mut g);

        let pow = u.val().powf(g.val() - 1.0);

        target.der.set(pow * (g.der() * u.val().ln() * u.val() + g.val() * u.der()));
        target.val = pow * u.val();
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
