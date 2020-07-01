use super::pos::Pos;

macro_rules! token {
    (
    $(#[$container_meta:meta])*
    pub enum Token<'src> {
    $(
        $(#[$variant_meta:meta])*
        $variant:ident ($struct:tt<'src>) as $display:expr,
    )*
    }) => {
        $(#[$container_meta])*
        pub enum Token<'src> {
        $(
            $(#[$variant_meta])*
            $variant($struct<'src>),
        )*
        }

        impl<'src> Token<'src> {
            pub fn kind(&self) -> TokenKind {
                match self {
                $(
                    Token::$variant(_) => TokenKind::$variant,
                )*
                }
            }

            pub fn display(&self) -> &'static str {
                match self {
                $(
                    Token::$variant(_) => $display,
                )*
                }
            }

            pub fn pos(&self) -> Pos<'src> {
                match self {
                $(
                    Token::$variant(token) => token.pos,
                )*
                }
            }
        }

        impl<'src> std::fmt::Display for Token<'src> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.display())
            }
        }

        $(
        impl<'src> From<$struct<'src>> for Token<'src> {
            fn from(token: $struct<'src>) -> Self {
                Token::$variant(token)
            }
        }
        )*

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TokenKind {
        $(
            $variant,
        )*
        }

        impl TokenKind {
            pub fn display(&self) -> &'static str {
                match self {
                $(
                    TokenKind::$variant => $display,
                )*
                }
            }
        }

        impl std::fmt::Display for TokenKind {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.display())
            }
        }

        $(
        impl<'src> $struct<'src> {
            pub fn display() -> &'static str {
                $display
            }
        }
        )*

    };
}

token! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    /// "+"
    Add(Add<'src>) as "+",
    /// "-"
    Sub(Sub<'src>) as "-",
    /// "*"
    Mul(Mul<'src>) as "*",
    /// "/"
    Div(Div<'src>) as "/",
    /// "<"
    Lt(Lt<'src>) as "<",
    /// "<="
    Lte(Lte<'src>) as "<=",
    /// ">"
    Gt(Gt<'src>) as ">",
    /// ">="
    Gte(Gte<'src>) as ">=",
    /// "=="
    Eq(Eq<'src>) as "==",
    /// "!="
    Neq(Neq<'src>) as "!=",
    /// "="
    Assign(Assign<'src>) as "=",

    /// "("
    ParenLeft(ParenLeft<'src>) as "(",
    /// ")"
    ParenRight(ParenRight<'src>) as ")",
    /// "{"
    BraceLeft(BraceLeft<'src>) as "{",
    /// "}"
    BraceRight(BraceRight<'src>) as "}",

    /// 数値リテラル
    Num(Num<'src>) as "number",

    /// 識別子（変数名とか）
    Ident(Ident<'src>) as "identifier",

    /// "return" keyword
    Return(Return<'src>) as "return",
    /// "if" keyword
    If(If<'src>) as "if",
    /// "else" keyword
    Else(Else<'src>) as "else",
    /// "while" keyword
    While(While<'src>) as "while",
    /// ";"
    Semi(Semi<'src>) as ";",
    /// ","
    Comma(Comma<'src>) as ",",
}
}

macro_rules! plain_token {
    ($name:tt) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<'src> {
            pub pos: Pos<'src>,
        }

        impl<'src> $name<'src> {
            pub fn new(pos: Pos<'src>) -> Self {
                $name { pos }
            }
        }
    };

    ($name:tt $(, $rmn:tt )+) => {
        plain_token!($name);
        $(
            plain_token!($rmn);
        )+
    }
}

plain_token!(Add, Sub, Mul, Div, Lt, Lte, Gt, Gte, Eq, Neq, Assign);
plain_token!(ParenLeft, ParenRight, BraceLeft, BraceRight);
plain_token!(Return, If, Else, While, Semi, Comma);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Num<'src> {
    pub num: usize,
    pub pos: Pos<'src>,
}

impl<'src> Num<'src> {
    pub fn new(num: usize, pos: Pos<'src>) -> Self {
        Num { num, pos }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident<'src> {
    pub name: &'src str,
    pub pos: Pos<'src>,
}

impl<'src> Ident<'src> {
    pub fn new(name: &'src str, pos: Pos<'src>) -> Self {
        Ident { name, pos }
    }
}
