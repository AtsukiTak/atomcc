#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub origin: &'a str,
    pub pos: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// 演算子
    Op(Op),
    /// "(" or ")"
    Par(Par),
    /// 数値リテラル
    Num(usize),
    /// 識別子（変数名とか）
    Ident(&'a str),
    /// キーワード（ifとかreturnとか）
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    // 算術演算子
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    // 比較演算子
    Lt,  // <
    Lte, // <=
    Gt,  // >
    Gte, // >=
    Eq,  // ==
    Neq, // !=

    // 代入演算子
    Assign, // =
}

/// Parentheses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Par {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// "return" keyword
    Return,
    /// "if" keyword
    If,
    /// "else" keyword
    Else,
    /// "while" keyword
    While,
    /// ";"
    Semi,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, origin: &'a str, pos: usize) -> Token<'a> {
        Token { kind, origin, pos }
    }

    pub fn op(&self) -> Option<Op> {
        match self.kind {
            TokenKind::Op(op) => Some(op),
            _ => None,
        }
    }

    pub fn expect_op(&self) -> Op {
        self.op()
            .unwrap_or_else(|| self.exit_with_err_msg("not an operator"))
    }

    pub fn par(&self) -> Option<Par> {
        match self.kind {
            TokenKind::Par(par) => Some(par),
            _ => None,
        }
    }

    pub fn expect_par(&self) -> Par {
        self.par()
            .unwrap_or_else(|| self.exit_with_err_msg("not a parentheses"))
    }

    pub fn num(&self) -> Option<usize> {
        match self.kind {
            TokenKind::Num(n) => Some(n),
            _ => None,
        }
    }

    pub fn expect_num(&self) -> usize {
        self.num()
            .unwrap_or_else(|| self.exit_with_err_msg("not a number"))
    }

    pub fn ident(&self) -> Option<&'a str> {
        match self.kind {
            TokenKind::Ident(s) => Some(s),
            _ => None,
        }
    }

    pub fn exit_with_err_msg(&self, msg: &'static str) -> ! {
        eprintln!("{}", self.origin);
        let leading_spaces = " ".repeat(self.pos);
        eprintln!("{}^ {}", leading_spaces, msg);
        std::process::exit(1)
    }
}
