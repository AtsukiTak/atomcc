pub struct Token<'a> {
    kind: TokenKind,
    origin: &'a str,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Op(Op),
    Par(Par),
    Num(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// Parentheses
#[derive(Debug, Clone, Copy)]
pub enum Par {
    Left,
    Right,
}

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
    fn new(kind: TokenKind, origin: &'a str, pos: usize) -> Token {
        Token { kind, origin, pos }
    }

    fn new_num(n: usize, origin: &'a str, pos: usize) -> Token {
        Token {
            kind: TokenKind::Num(n),
            origin,
            pos,
        }
    }

    pub fn expect_op(&self) -> Op {
        match self.kind {
            TokenKind::Op(op) => op,
            _ => exit_with_err_msg(self.origin, self.pos, "not an operator"),
        }
    }

    pub fn expect_num(&self) -> usize {
        match self.kind {
            TokenKind::Num(n) => n,
            _ => exit_with_err_msg(self.origin, self.pos, "not a number"),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_s(self.s.trim_start());
        if self.s.is_empty() {
            return None;
        }

        if let Some(kind) = match self.s.as_bytes()[0] {
            b'+' => Some(TokenKind::Op(Op::Add)),
            b'-' => Some(TokenKind::Op(Op::Sub)),
            b'*' => Some(TokenKind::Op(Op::Mul)),
            b'/' => Some(TokenKind::Op(Op::Div)),
            b'(' => Some(TokenKind::Par(Par::Left)),
            b')' => Some(TokenKind::Par(Par::Right)),
            _ => None,
        } {
            let token = Token::new(kind, self.origin, self.pos);
            self.update_s(self.s.split_at(1).1);
            return Some(token);
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            let digit = usize::from_str_radix(digit_s, 10).unwrap();
            let token = Token::new_num(digit, self.origin, self.pos);
            self.update_s(remain_s);
            return Some(token);
        }

        exit_with_err_msg(self.origin, self.pos, "Unable to tokenize")
    }
}

impl<'a> TokenIter<'a> {
    fn update_s(&mut self, new_s: &'a str) {
        self.pos += self.s.len() - new_s.len();
        self.s = new_s;
    }
}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

/// Print error messages such as
/// "1 + 3 ++"
/// "       ^ not number"
fn exit_with_err_msg(origin: &str, pos: usize, msg: &str) -> ! {
    eprintln!("{}", origin);
    let leading_spaces = " ".repeat(pos);
    eprintln!("{}^ {}", leading_spaces, msg);
    std::process::exit(1)
}
