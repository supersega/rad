use std::ops::Fn;

use crate::Dual;

pub fn gradient<F>(f: F, arg: &[Dual]) 
    where F: Fn(&[Dual]) -> Dual,
{
    let _grad: Vec<Dual> = arg.iter().map(|x| { 
        x.seed(); 
        let dfdx = f(arg);
        x.unseed(); 
        dfdx 
    }).collect();
}

pub fn gradient1<F>(f: F, arg: &[Dual]) 
    where F: Fn(&[Dual]) -> Dual
{
    let _grad: Vec<Dual> = arg.iter().map(|x| { 
        x.seed();
        let dfdx = f(arg);
        x.unseed();
        dfdx
    }).collect();
}

pub struct WrtPack<F> 
    where F: Fn() -> Dual
{
    f: F,
}

// impl WrtPack<'a, F> 
//     where F: Fn() -> Dual + 'a 
// {
//     pub fn wrt<Args>(w: Args) {

//     }
// }

// pub fn gradient2<F>(f: F, arg1: &[Dual], arg2: &[Dual]) -> WrtPack<_>
//     where F: Fn(&[Dual], &[Dual]) -> Dual,
// {
//     WrtPack { f: || f(arg1, arg2) }
// }

#[cfg(test)]
mod test {
use super::*;
#[test]
fn test_grad() {
    let x = vec![Dual::from(1.0), Dual::from(1.0)];
    let f = |x: &[Dual]| -> Dual { (x[0] + x[1]).into() };
    gradient(f, &x);
}
#[test]
fn test_grad2() {
    let x = vec![Dual::from(1.0), Dual::from(1.0)];
    let y = vec![Dual::from(1.0), Dual::from(1.0)];
    let f = |x: &[Dual], y: &[Dual]| -> Dual { (x[0] + y[1]).into() };
    //gradient2(f, &x, &y);
    let w = WrtPack { f: || f(&x, &y) };
    (w.f)();
}
}
