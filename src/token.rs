#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    origin: &'a str,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<'a> {
    Op(Op),
    /// "(", ")"
    Par(Par),
    Num(usize),
    /// 識別子（変数名とか）
    Ident(&'a str),
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
        self.parse_inner().map(|(token, _)| token)
    }

    fn parse_inner(&self) -> Option<(Token<'a>, &'a str)> {
        let s = self.s.trim_start();
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
                return Some((token, rmn));
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
            return Some((token, rmn));
        }

        // 数値リテラルを調べる
        let (digit_s, remain_s) = split_digit(s);
        if !digit_s.is_empty() {
            let digit = usize::from_str_radix(digit_s, 10).unwrap();
            let token = Token::new_num(digit, self.origin, self.pos);
            return Some((token, remain_s));
        }

        // 識別子を調べる
        if let Some(token) = s.split_whitespace().next() {
            let (_, rmn) = s.split_at(token.len());
            let token = Token::new(TokenKind::Ident(token), self.origin, self.pos);
            return Some((token, rmn));
        }

        self.exit_with_err_msg("Unable to tokenize")
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
        match self.parse_inner() {
            None => None,
            Some((token, next_s)) => {
                self.update_s(next_s);
                Some(token)
            }
        }
    }
}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
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
