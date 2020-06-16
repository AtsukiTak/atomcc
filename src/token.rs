#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    origin: &'a str,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<'a> {
    /// 演算子
    Op(Op),
    /// "(" or ")"
    Par(Par),
    /// 数値リテラル
    Num(usize),
    /// 識別子（変数名とか）
    Ident(&'a str),
    /// "return" keyword
    Return,
    /// "if" keyword
    If,
    /// "else" keyword
    Else,
    /// ";"
    Semi,
}

#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum Par {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenIter<'a> {
    origin: &'a str,
    s: &'a str,
    // 現在の文字が全体の何文字目か
    pos: usize,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter {
        origin: s,
        s,
        pos: 0,
    }
}

impl<'a> Token<'a> {
    fn new(kind: TokenKind<'a>, origin: &'a str, pos: usize) -> Token<'a> {
        Token { kind, origin, pos }
    }

    fn new_num(n: usize, origin: &'a str, pos: usize) -> Token {
        Token {
            kind: TokenKind::Num(n),
            origin,
            pos,
        }
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
        exit_with_err_msg(self.origin, self.pos, msg)
    }
}

impl<'a> TokenIter<'a> {
    pub fn peek(&self) -> Option<Token<'a>> {
        let mut copied = *self;
        copied.next()
    }

    fn update_s(&mut self, next_s: &'a str) {
        self.pos += self.s.len() - next_s.len();
        self.s = next_s;
    }

    pub fn exit_with_err_msg(&self, msg: &'static str) -> ! {
        exit_with_err_msg(self.origin, self.pos, msg)
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_s(self.s.trim_start());

        let s = self.s;

        if s.is_empty() {
            return None;
        }

        // 2文字の演算子を調べる
        if s.len() >= 2 {
            let (token, rmn) = s.split_at(2);
            if let Some(kind) = match token {
                "<=" => Some(TokenKind::Op(Op::Lte)),
                ">=" => Some(TokenKind::Op(Op::Gte)),
                "==" => Some(TokenKind::Op(Op::Eq)),
                "!=" => Some(TokenKind::Op(Op::Neq)),
                _ => None,
            } {
                let token = Token::new(kind, self.origin, self.pos);
                self.update_s(rmn);
                return Some(token);
            }
        }

        // 1文字のトークンを調べる
        let (token, rmn) = s.split_at(1);
        if let Some(kind) = match token.as_bytes()[0] {
            b'+' => Some(TokenKind::Op(Op::Add)),
            b'-' => Some(TokenKind::Op(Op::Sub)),
            b'*' => Some(TokenKind::Op(Op::Mul)),
            b'/' => Some(TokenKind::Op(Op::Div)),
            b'<' => Some(TokenKind::Op(Op::Lt)),
            b'>' => Some(TokenKind::Op(Op::Gt)),
            b'=' => Some(TokenKind::Op(Op::Assign)),
            b'(' => Some(TokenKind::Par(Par::Left)),
            b')' => Some(TokenKind::Par(Par::Right)),
            b';' => Some(TokenKind::Semi),
            _ => None,
        } {
            let token = Token::new(kind, self.origin, self.pos);
            self.update_s(rmn);
            return Some(token);
        }

        // 数値リテラルを調べる
        if let Some((digit, rmn)) = split_digit(s) {
            let token = Token::new_num(digit, self.origin, self.pos);
            self.update_s(rmn);
            return Some(token);
        }

        // キーワード/識別子を調べる
        if let Some(token) = s.split_whitespace().next() {
            let kind = match token {
                "return" => TokenKind::Return,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                ident => TokenKind::Ident(ident),
            };
            let (_, rmn) = s.split_at(token.len());
            let token = Token::new(kind, self.origin, self.pos);
            self.update_s(rmn);
            return Some(token);
        }

        self.exit_with_err_msg("Unable to tokenize")
    }
}

fn split_digit(s: &str) -> Option<(usize, &str)> {
    let first_non_num_idx = s.find(|c| !char::is_digit(c, 10)).unwrap_or(s.len());
    if first_non_num_idx == 0 {
        None
    } else {
        let (digit_s, rmn) = s.split_at(first_non_num_idx);
        Some((usize::from_str_radix(digit_s, 10).unwrap(), rmn))
    }
}

/// Print error messages such as
/// "1 + 3 ++"
/// "       ^ not number"
pub fn exit_with_err_msg(origin: &str, pos: usize, msg: &str) -> ! {
    eprintln!("{}", origin);
    let leading_spaces = " ".repeat(pos);
    eprintln!("{}^ {}", leading_spaces, msg);
    std::process::exit(1)
}
