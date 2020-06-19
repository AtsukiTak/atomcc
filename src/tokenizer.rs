use crate::token::{Keyword, Op, Par, Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub struct TokenStream<'a> {
    origin: &'a str,
    s: &'a str,
    // 現在の文字が全体の何文字目か
    pos: usize,
}

pub fn tokenize<'a>(s: &'a str) -> TokenStream<'a> {
    TokenStream {
        origin: s,
        s,
        pos: 0,
    }
}

impl<'a> TokenStream<'a> {
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

impl<'a> Iterator for TokenStream<'a> {
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
            b';' => Some(TokenKind::Keyword(Keyword::Semi)),
            _ => None,
        } {
            let token = Token::new(kind, self.origin, self.pos);
            self.update_s(rmn);
            return Some(token);
        }

        // 数値リテラルを調べる
        if let Some((digit, rmn)) = split_digit(s) {
            let token = Token::new(TokenKind::Num(digit), self.origin, self.pos);
            self.update_s(rmn);
            return Some(token);
        }

        // キーワード/識別子を調べる
        let (token, rmn) = split_delim(s);
        let kind = match token {
            "return" => TokenKind::Keyword(Keyword::Return),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "while" => TokenKind::Keyword(Keyword::While),
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

    let delimiters = [
        ' ', '{', '}', '(', ')', '=', ';', '+', '-', '*', '/', '<', '>',
    ];

    let idx = s.find(&delimiters[..]).unwrap_or(s.len());
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

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind as TK;

    fn assert_tk<'a>(input: &'a str, expected: Vec<TokenKind<'a>>) {
        let found = tokenize(input).map(|token| token.kind).collect::<Vec<_>>();
        assert_eq!(found, expected);
    }

    #[test]
    fn tokenizer_tests() {
        assert_tk("", vec![]);
        assert_tk("   ", vec![]);
        assert_tk("42", vec![TK::Num(42)]);
        assert_tk("-42", vec![TK::Op(Op::Sub), TK::Num(42)]);
        assert_tk("   42   ", vec![TK::Num(42)]);
        assert_tk("42+2", vec![TK::Num(42), TK::Op(Op::Add), TK::Num(2)]);
        assert_tk("ho_ge", vec![TK::Ident("ho_ge")]);
        assert_tk("hoge42", vec![TK::Ident("hoge42")]);
        assert_tk("i<3", vec![TK::Ident("i"), TK::Op(Op::Lt), TK::Num(3)]);
        assert_tk(
            "hoge+42",
            vec![TK::Ident("hoge"), TK::Op(Op::Add), TK::Num(42)],
        );
        assert_tk(
            "hoge=42",
            vec![TK::Ident("hoge"), TK::Op(Op::Assign), TK::Num(42)],
        );
        assert_tk(
            "if(42",
            vec![TK::Keyword(Keyword::If), TK::Par(Par::Left), TK::Num(42)],
        );
        assert_tk("hoge;", vec![TK::Ident("hoge"), TK::Keyword(Keyword::Semi)]);
        assert_tk(
            ")else hoge",
            vec![
                TK::Par(Par::Right),
                TK::Keyword(Keyword::Else),
                TK::Ident("hoge"),
            ],
        );
        assert_tk(
            "while (",
            vec![TK::Keyword(Keyword::While), TK::Par(Par::Left)],
        );
    }
}
