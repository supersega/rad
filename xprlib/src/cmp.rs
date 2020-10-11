use crate::{
    dual::Dual,
    xpr::expression::{Xpr, XprWrapper},
};
use std::cmp::{Ordering, PartialEq};

impl PartialEq for Dual {
    fn eq(&self, other: &Dual) -> bool {
        self.val.eq(&other.val)
    }
}

impl<T> PartialEq<XprWrapper<T>> for Dual
where
    T: Xpr,
{
    fn eq(&self, other: &XprWrapper<T>) -> bool {
        self.eq(&Dual::from(*other))
    }
}

impl<T> PartialEq<Dual> for XprWrapper<T>
where
    T: Xpr,
{
    fn eq(&self, other: &Dual) -> bool {
        Dual::from(*self).eq(other)
    }
}

impl<L, R> PartialEq<XprWrapper<R>> for XprWrapper<L>
where
    L: Xpr,
    R: Xpr,
{
    fn eq(&self, other: &XprWrapper<R>) -> bool {
        self.eq(&Dual::from(*other))
    }
}

impl PartialOrd for Dual {
    fn partial_cmp(&self, other: &Dual) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T> PartialOrd<XprWrapper<T>> for Dual
where
    T: Xpr,
{
    fn partial_cmp(&self, other: &XprWrapper<T>) -> Option<Ordering> {
        self.partial_cmp(&Dual::from(*other))
    }
}

impl<T> PartialOrd<Dual> for XprWrapper<T>
where
    T: Xpr,
{
    fn partial_cmp(&self, other: &Dual) -> Option<Ordering> {
        Dual::from(*self).partial_cmp(other)
    }
}

impl<L, R> PartialOrd<XprWrapper<R>> for XprWrapper<L>
where
    L: Xpr,
    R: Xpr,
{
    fn partial_cmp(&self, other: &XprWrapper<R>) -> Option<Ordering> {
        self.partial_cmp(&Dual::from(*other))
    }
}
