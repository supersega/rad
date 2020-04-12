/// Operation type. We use them for binary
/// operations, and for assign operation
#[derive(Copy, Clone, Debug)]
pub enum Op {
    /// Add operation
    Add,
    /// Sub operation
    Sub,
}

/// Unary expression type
#[derive(Copy, Clone, Debug)]
pub enum UnOp {
    /// Negate expression
    Neg,
}
