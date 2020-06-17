use crate::token::{Op, Par, Token, TokenKind};

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
        let (token, rmn) = split_delim(s);
        let kind = match token {
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            ident => TokenKind::Ident(ident),
        };
        let token = Token::new(kind, self.origin, self.pos);
        self.update_s(rmn);
        return Some(token);
    }
}

// 先頭から数値を読み込む
// "42world" -> (42, "world")
fn split_digit(s: &str) -> Option<(usize, &str)> {
    let first_non_num_idx = s.find(|c| !char::is_digit(c, 10)).unwrap_or(s.len());
    if first_non_num_idx == 0 {
        None
    } else {
        let (digit_s, rmn) = s.split_at(first_non_num_idx);
        Some((usize::from_str_radix(digit_s, 10).unwrap(), rmn))
    }
}

// 特定のdelimiterで区切った文字列を返す。
// delimiterは、
// - whitespace, "{", "}", "(", ")"
fn split_delim(s: &str) -> (&str, &str) {
    assert!(s.len() != 0);

    let idx = s.find(&[' ', '{', '}', '(', ')'][..]).unwrap_or(s.len());
    if idx == 0 {
        // '{' などを返す
        s.split_at(1)
    } else {
        s.split_at(idx)
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
