use super::{pos::Pos, token::*};

#[derive(Debug, Clone, Copy)]
pub struct TokenStream<'src> {
    s: &'src str,
    // ソースコード上における現在の文字の位置
    pub pos: Pos<'src>,
}

pub fn tokenize<'src>(src: &'src str) -> TokenStream<'src> {
    TokenStream {
        s: src,
        pos: Pos::new(src),
    }
}

impl<'src> TokenStream<'src> {
    pub fn peek(&self) -> Option<Token<'src>> {
        let mut copied = *self;
        copied.next()
    }

    fn update_s(&mut self, next_s: &'src str) {
        self.pos += self.s.len() - next_s.len();
        self.s = next_s;
    }

    pub fn exit_with_err_msg(&self, msg: &'static str) -> ! {
        eprintln!("{}", self.pos.display(msg));
        std::process::exit(1)
    }
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_s(self.s.trim_start());

        let s = self.s;

        if s.is_empty() {
            return None;
        }

        // 2文字の演算子を調べる
        if s.len() >= 2 {
            let (token_str, rmn) = s.split_at(2);
            if let Some(token) = match token_str {
                "<=" => Some(Token::Lte(Lte::new(self.pos))),
                ">=" => Some(Token::Gte(Gte::new(self.pos))),
                "==" => Some(Token::Eq(Eq::new(self.pos))),
                "!=" => Some(Token::Neq(Neq::new(self.pos))),
                _ => None,
            } {
                self.update_s(rmn);
                return Some(token);
            }
        }

        // 1文字のトークンを調べる
        let (token_str, rmn) = s.split_at(1);
        if let Some(token) = match token_str.as_bytes()[0] {
            b'+' => Some(Token::Add(Add::new(self.pos))),
            b'-' => Some(Token::Sub(Sub::new(self.pos))),
            b'*' => Some(Token::Mul(Mul::new(self.pos))),
            b'/' => Some(Token::Div(Div::new(self.pos))),
            b'<' => Some(Token::Lt(Lt::new(self.pos))),
            b'>' => Some(Token::Gt(Gt::new(self.pos))),
            b'=' => Some(Token::Assign(Assign::new(self.pos))),
            b'(' => Some(Token::ParenLeft(ParenLeft::new(self.pos))),
            b')' => Some(Token::ParenRight(ParenRight::new(self.pos))),
            b'{' => Some(Token::BraceLeft(BraceLeft::new(self.pos))),
            b'}' => Some(Token::BraceRight(BraceRight::new(self.pos))),
            b';' => Some(Token::Semi(Semi::new(self.pos))),
            _ => None,
        } {
            self.update_s(rmn);
            return Some(token);
        }

        // 数値リテラルを調べる
        if let Some((digit, rmn)) = split_digit(s) {
            let token = Token::Num(Num::new(digit, self.pos));
            self.update_s(rmn);
            return Some(token);
        }

        // キーワード/識別子を調べる
        let (token_str, rmn) = split_delim(s);
        let token = match token_str {
            "return" => Token::Return(Return::new(self.pos)),
            "if" => Token::If(If::new(self.pos)),
            "else" => Token::Else(Else::new(self.pos)),
            "while" => Token::While(While::new(self.pos)),
            ident => Token::Ident(Ident::new(ident, self.pos)),
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind as Kind;

    fn assert_kind<'src>(input: &'src str, expected: Vec<Kind>) {
        let found = tokenize(input)
            .map(|token| token.kind())
            .collect::<Vec<_>>();
        assert_eq!(found, expected);
    }

    #[test]
    fn token_kind_test() {
        assert_kind("", vec![]);
        assert_kind("   ", vec![]);
        assert_kind("42", vec![Kind::Num]);
        assert_kind("(", vec![Kind::ParenLeft]);
        assert_kind("}", vec![Kind::BraceRight]);
        assert_kind("-42", vec![Kind::Sub, Kind::Num]);
        assert_kind("   42   ", vec![Kind::Num]);
        assert_kind("42+2", vec![Kind::Num, Kind::Add, Kind::Num]);
        assert_kind("ho_ge", vec![Kind::Ident]);
        assert_kind("hoge42", vec![Kind::Ident]);
        assert_kind("i<3", vec![Kind::Ident, Kind::Lt, Kind::Num]);
        assert_kind("hoge+42", vec![Kind::Ident, Kind::Add, Kind::Num]);
        assert_kind("hoge=42", vec![Kind::Ident, Kind::Assign, Kind::Num]);
        assert_kind("if(42", vec![Kind::If, Kind::ParenLeft, Kind::Num]);
        assert_kind("hoge;", vec![Kind::Ident, Kind::Semi]);
        assert_kind(
            ")else hoge",
            vec![Kind::ParenRight, Kind::Else, Kind::Ident],
        );
        assert_kind("while (", vec![Kind::While, Kind::ParenLeft]);
    }
}
