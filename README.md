# RAD - Rust Automatic Differentiation Library

[![codecov](https://codecov.io/gh/supersega/rad/branch/master/graph/badge.svg)](https://codecov.io/gh/supersega/rad)

## Scalar function derivative evaluation
This example shows how to compute derivative of simple function.
```rust
// Import Dual number and derivative macro.
use rad::{Dual, derivative};

fn main() {
	// Here f - function which derivative we want to eval.
	let f =  |x: Dual|  -> Dual { (x * x).into() };
	// Create dual number using From trait.
	let x: Dual = 3.0.into();
	// Calculate derivative of f w.r.t. x.
	let dfdx = derivative!(f(x), x);
	// Check derivative value.
	assert_eq!(dfdx, 6.0);
}
```
