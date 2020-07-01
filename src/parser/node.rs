use super::op::BinOp;
use crate::token::token::*;

#[derive(Debug, Clone)]
pub enum Stmt<'src> {
    Assign(StmtAssign<'src>),
    Block(StmtBlock<'src>),
    Expr(Expr<'src>),
    Return(StmtReturn<'src>),
    If(StmtIf<'src>),
    While(StmtWhile<'src>),
}

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Num(Num<'src>),
    Ident(ExprIdent<'src>),
    Call(ExprCall<'src>),
    BinOp(ExprBinOp<'src>),
    Paren(ExprParen<'src>),
}

/// "hoge = 42;"
#[derive(Debug, Clone)]
pub struct StmtAssign<'src> {
    pub lhs_offset: usize,
    pub lhs: Ident<'src>,
    pub assign_token: Assign<'src>,
    pub rhs: Expr<'src>,
    pub semi_token: Semi<'src>,
}

/// "{ hoge = 42; return hoge; }"
#[derive(Debug, Clone)]
pub struct StmtBlock<'src> {
    pub brace_left_token: BraceLeft<'src>,
    pub stmts: Vec<Stmt<'src>>,
    pub brace_right_token: BraceRight<'src>,
}

/// "return 42;"
#[derive(Debug, Clone)]
pub struct StmtReturn<'src> {
    pub return_token: Return<'src>,
    pub expr: Expr<'src>,
    pub semi_token: Semi<'src>,
}

/// "if (true) { 42 }"
/// "if (i = 0) { 42 } else { 24 }"
#[derive(Debug, Clone)]
pub struct StmtIf<'src> {
    pub if_token: If<'src>,
    pub paren_left_token: ParenLeft<'src>,
    pub cond: Expr<'src>,
    pub paren_right_token: ParenRight<'src>,
    pub then_branch: Box<Stmt<'src>>,
    pub else_branch: Option<(Else<'src>, Box<Stmt<'src>>)>,
}

/// "while (i < 10) { i = i + 1 }"
#[derive(Debug, Clone)]
pub struct StmtWhile<'src> {
    pub while_token: While<'src>,
    pub paren_left_token: ParenLeft<'src>,
    pub cond: Expr<'src>,
    pub paren_right_token: ParenRight<'src>,
    pub block: Box<Stmt<'src>>,
}

/// "hoge"
#[derive(Debug, Clone)]
pub struct ExprIdent<'src> {
    pub ident_offset: usize,
    pub ident: Ident<'src>,
}

/// "func()"
#[derive(Debug, Clone)]
pub struct ExprCall<'src> {
    pub ident: Ident<'src>,
    pub paren_left_token: ParenLeft<'src>,
    pub params: Vec<Expr<'src>>,
    pub paren_right_token: ParenRight<'src>,
}

/// "4 * 2"
#[derive(Debug, Clone)]
pub struct ExprBinOp<'src> {
    pub lhs: Box<Expr<'src>>,
    pub op: BinOp<'src>,
    pub rhs: Box<Expr<'src>>,
}

#[derive(Debug, Clone)]
pub struct ExprParen<'src> {
    pub paren_left_token: ParenLeft<'src>,
    pub expr: Box<Expr<'src>>,
    pub paren_right_token: ParenRight<'src>,
}
