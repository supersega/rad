[![Build Status](https://travis-ci.org/supersega/rad.svg?branch=master)](https://travis-ci.org/github/supersega/rad)
[![codecov](https://codecov.io/gh/supersega/rad/branch/master/graph/badge.svg)](https://codecov.io/gh/supersega/rad)

# RAD - Rust Automatic Differentiation Library
This library is inspired by C++ library [autodiff](https://github.com/autodiff/autodiff) crated by [Allan Leal](https://github.com/allanleal).

## Scalar function derivative evaluation
This example shows how to compute derivative of simple function.
```rust
// Import Dual number and derivative macro.
use rad::{Dual, derivative};

fn main() {
    // Here f - function which derivative we want to eval.
    let f = x: Dual| -> Dual { (x * x).into() };
    // Create dual number using From trait.
    let x: Dual = 3.0.into();
    // Calculate derivative of f w.r.t. x.
    let dfdx = derivative!(f(x), x);
    // Check derivative value.
    assert_eq!(dfdx, 6.0);
}
```
