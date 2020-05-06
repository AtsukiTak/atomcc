#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Num(usize),
}

pub struct TokenIter<'a> {
    s: &'a str,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s }
}

impl Token {
    pub fn expect_num(&self) -> usize {
        match self {
            Token::Num(n) => *n,
            t => panic!("Expect number but found {:?}", t),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }

        if self.s.as_bytes()[0] == b'+' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Plus);
        }

        if self.s.as_bytes()[0] == b'-' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Minus);
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(usize::from_str_radix(digit_s, 10).unwrap()));
        }

        panic!("Invalid token stream")
    }
}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}
