use std::cmp::{PartialEq, Ordering};
use crate::xpr::assign::Assign;
use crate::dual::Dual;
use crate::xpr::expression::{XprWrapper, Xpr};

impl PartialEq for Dual {
    fn eq(&self, other: &Dual) -> bool {
        self.val.eq(&other.val)
    }
}

impl<T: Xpr + Assign + Copy + Clone> PartialEq<XprWrapper<T>> for Dual {
    fn eq(&self, other: &XprWrapper<T>) -> bool {
        self.eq(&Dual::from(*other))
    }
}

impl<T: Xpr + Assign + Copy + Clone> PartialEq<Dual> for XprWrapper<T> {
    fn eq(&self, other: &Dual) -> bool {
        Dual::from(*self).eq(other)
    }
}

impl<L, R> PartialEq<XprWrapper<R>> for XprWrapper<L> 
where L: Xpr + Assign + Copy + Clone,
      R: Xpr + Assign + Copy + Clone
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

impl<T: Xpr + Assign + Copy + Clone> PartialOrd<XprWrapper<T>> for Dual {
    fn partial_cmp(&self, other: &XprWrapper<T>) -> Option<Ordering> {
        self.partial_cmp(&Dual::from(*other))
    }
}

impl<T: Xpr + Assign + Copy + Clone> PartialOrd<Dual> for XprWrapper<T> {
    fn partial_cmp(&self, other: &Dual) -> Option<Ordering> {
        Dual::from(*self).partial_cmp(other)
    }
}

impl<L, R> PartialOrd<XprWrapper<R>> for XprWrapper<L> 
where L: Xpr + Assign + Copy + Clone,
      R: Xpr + Assign + Copy + Clone
{
    fn partial_cmp(&self, other: &XprWrapper<R>) -> Option<Ordering> {
        self.partial_cmp(&Dual::from(*other))
    }
}
