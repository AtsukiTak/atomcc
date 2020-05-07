pub struct Token {
    kind: TokenKind,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Op(Op),
    Num(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
}

pub struct TokenIter<'a> {
    s: &'a str,
    // 現在の文字が全体の何文字目か
    n_bytes: usize,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s, n_bytes: 0 }
}

impl Token {
    fn new_num(n: usize, pos: usize) -> Token {
        Token {
            kind: TokenKind::Num(n),
            pos,
        }
    }

    fn new_op(op: Op, pos: usize) -> Token {
        Token {
            kind: TokenKind::Op(op),
            pos,
        }
    }

    pub fn expect_op(&self) -> Op {
        match self.kind {
            TokenKind::Op(op) => op,
            t => panic!("Expect operator but found {:?}", t),
        }
    }

    pub fn expect_num(&self) -> usize {
        match self.kind {
            TokenKind::Num(n) => n,
            t => panic!("Expect number but found {:?}", t),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_s(self.s.trim_start());
        if self.s.is_empty() {
            return None;
        }

        if self.s.as_bytes()[0] == b'+' {
            self.update_s(self.s.split_at(1).1);
            return Some(Token::new_op(Op::Plus, self.n_bytes));
        }

        if self.s.as_bytes()[0] == b'-' {
            self.update_s(self.s.split_at(1).1);
            return Some(Token::new_op(Op::Minus, self.n_bytes));
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.update_s(remain_s);
            let digit = usize::from_str_radix(digit_s, 10).unwrap();
            return Some(Token::new_num(digit, self.n_bytes));
        }

        panic!("Invalid token stream")
    }
}

impl<'a> TokenIter<'a> {
    fn update_s(&mut self, new_s: &'a str) {
        self.n_bytes += self.s.len() - new_s.len();
        self.s = new_s;
    }
}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}
