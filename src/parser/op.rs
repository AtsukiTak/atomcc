use crate::token::token::*;

/// A binary operator: `+`, `+=`, `<`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp<'src> {
    Add(Add<'src>),
    Sub(Sub<'src>),
    Mul(Mul<'src>),
    Div(Div<'src>),
    Lt(Lt<'src>),
    Lte(Lte<'src>),
    Gt(Gt<'src>),
    Gte(Gte<'src>),
    Eq(Eq<'src>),
    Neq(Neq<'src>),
    Assign(Assign<'src>),
}
