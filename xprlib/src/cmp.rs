use std::cmp::PartialEq;
use crate::xpr::assign::Assign;
use crate::dual::Dual;
use crate::xpr::expression::{XprWrapper, Xpr};

impl<T: Xpr + Assign + Copy + Clone> PartialEq<XprWrapper<T>> for Dual {
    fn eq(&self, other: &XprWrapper<T>) -> bool {
        *self == Dual::from(*other)
    }
}

impl<T: Xpr + Assign + Copy + Clone> PartialEq<Dual> for XprWrapper<T> {
    fn eq(&self, other: &Dual) -> bool {
        Dual::from(*self) == *other
    }
}

impl<L: Xpr + Assign + Copy + Clone, R: Xpr + Assign + Copy + Clone> PartialEq<XprWrapper<R>> for XprWrapper<L> {
    fn eq(&self, other: &XprWrapper<R>) -> bool {
        Dual::from(*self) == *other
    }
}
