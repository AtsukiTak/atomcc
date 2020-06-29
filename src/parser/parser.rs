use super::{node::*, op::BinOp};
use crate::token::{new_tokenizer::TokenStream, token::*, Pos};
use std::collections::HashMap;

pub struct Parser<'src> {
    local_vars: HashMap<&'src str, usize>,
}

macro_rules! parse_exact {
    ($tokens:expr, $token:tt) => {
        match $tokens.next() {
            Some(Token::$token(token)) => token,
            Some(token) => {
                let msg = format!("expected {} but found {}", $token::display(), token);
                exit_with_err_msg(token.pos(), msg.as_str())
            }
            None => {
                let msg = format!("expected {} but found EOF", $token::display());
                exit_with_err_msg($tokens.pos, msg.as_str())
            }
        }
    };
}

impl<'src> Parser<'src> {
    pub fn new() -> Self {
        Parser {
            local_vars: HashMap::new(),
        }
    }

    fn offset_of_local_var(&mut self, ident: &'src str) -> usize {
        if let Some(offset) = self.local_vars.get(ident) {
            *offset
        } else {
            let offset = match self.local_vars.values().max() {
                Some(cur) => cur + 8,
                None => 0,
            };
            self.local_vars.insert(ident, offset);
            offset
        }
    }

    /// > program       = stmt*
    /// > stmt          = assign ";"
    ///     | "return" expr ";"
    ///     | "if" "(" expr ")" stmt ("else" stmt)?
    ///     | "while" "(" expr ")" stmt
    ///     | "{" stmt* "}"
    /// > assign        = (ident "=")? expr
    /// > expr          = equality
    /// > equality      = relational ("==" relational | "!=" relational)*
    /// > relational    = add ("<" add | "<=" add | ">" add | ">=" add)*
    /// > add           = mul ("+" mul | "-" mul)*
    /// > unary         = ("+" | "-")? primary
    /// > primary       = num
    ///     | ident ( "(" ")" )?
    ///     | "(" expr ")"
    ///
    /// で表現される文法をパースする関数。
    pub fn parse(&mut self, tokens: &mut TokenStream<'src>) -> Vec<Stmt<'src>> {
        self.parse_program(tokens)
    }

    /// > program       = stmt*
    ///
    /// で表現される非終端記号programをパースする関数。
    pub fn parse_program(&mut self, tokens: &mut TokenStream<'src>) -> Vec<Stmt<'src>> {
        let mut nodes = Vec::new();
        while let Some(_) = tokens.peek() {
            nodes.push(self.parse_stmt(tokens))
        }
        nodes
    }

    /// > stmt          = assign
    ///     | "return" expr ";"
    ///     | "if" "(" expr ")" stmt ("else" stmt)?
    ///     | "while" "(" expr ")" stmt
    ///     | "{" stmt* "}"
    ///
    /// で表現される非終端記号stmtをパースする関数。
    pub fn parse_stmt(&mut self, tokens: &mut TokenStream<'src>) -> Stmt<'src> {
        match tokens.peek() {
            // "return" から始まるとき
            Some(Token::Return(return_token)) => {
                let _ = tokens.next();
                let expr = self.parse_expr(tokens);
                let semi_token = parse_exact!(tokens, Semi);

                Stmt::Return(StmtReturn {
                    return_token,
                    expr,
                    semi_token,
                })
            }

            // "if" から始まるとき
            Some(Token::If(if_token)) => {
                let _ = tokens.next();

                // 次のTokenが "(" であることを確認
                let paren_left_token = parse_exact!(tokens, ParenLeft);

                // cond をパース
                let cond = self.parse_expr(tokens);

                // 次のTokenが ")" であることを確認
                let paren_right_token = parse_exact!(tokens, ParenRight);

                // stmt をパース
                let stmt = self.parse_stmt(tokens);

                // 次のTokenが "else" かどうか確認
                match tokens.peek() {
                    Some(Token::Else(else_token)) => {
                        let _ = tokens.next();
                        let else_stmt = self.parse_stmt(tokens);
                        Stmt::If(StmtIf {
                            if_token,
                            paren_left_token,
                            cond,
                            paren_right_token,
                            then_branch: Box::new(stmt),
                            else_branch: Some((else_token, Box::new(else_stmt))),
                        })
                    }
                    _ => Stmt::If(StmtIf {
                        if_token,
                        paren_left_token,
                        cond,
                        paren_right_token,
                        then_branch: Box::new(stmt),
                        else_branch: None,
                    }),
                }
            }

            // "while" から始まるとき
            Some(Token::While(while_token)) => {
                let _ = tokens.next();

                // 次のトークンが "(" であることを確認する
                let paren_left_token = parse_exact!(tokens, ParenLeft);

                let cond = self.parse_expr(tokens);

                // 次のトークンが ")" であることを確認する
                let paren_right_token = parse_exact!(tokens, ParenRight);

                let stmt = self.parse_stmt(tokens);

                Stmt::While(StmtWhile {
                    while_token,
                    paren_left_token,
                    cond,
                    paren_right_token,
                    block: Box::new(stmt),
                })
            }

            // "{" から始まるとき
            Some(Token::BraceLeft(brace_left_token)) => {
                let _ = tokens.next();

                let mut stmts = Vec::new();

                // "}" が現れるまでstmtをパースする
                let brace_right_token = loop {
                    match tokens.peek() {
                        Some(Token::BraceRight(token)) => break token,
                        Some(_) => stmts.push(self.parse_stmt(tokens)),
                        None => exit_with_err_msg(tokens.pos, "expected \"}\" but found EOF"),
                    }
                };

                // "}" を捨てる
                let _ = tokens.next();

                Stmt::Block(StmtBlock {
                    brace_left_token,
                    stmts,
                    brace_right_token,
                })
            }
            // その他の時はassignとして処理する
            _ => {
                let node = self.parse_assign(tokens);
                let semi_token = parse_exact!(tokens, Semi);
                node
            }
        }
    }

    /// > assign        = (ident "=")? expr ";"
    ///
    /// で表現される記号assignをパースする関数。
    pub fn parse_assign(&mut self, tokens: &mut TokenStream<'src>) -> Stmt<'src> {
        // 与えられたTokenStreamが (ident "=") で始まるかチェックする
        let mut tokens2 = *tokens;
        match (tokens2.next(), tokens2.next()) {
            (Some(Token::Ident(ident)), Some(Token::Assign(assign_token))) => {
                // (ident "=") で始まった場合のルート.
                // tokensを2つ進める。
                let _ = tokens.next();
                let _ = tokens.next();

                // ローカル変数のoffsetを求める
                let offset = self.offset_of_local_var(ident.name);

                let rhs = self.parse_expr(tokens);
                let semi_token = parse_exact!(tokens, Semi);

                Stmt::Assign(StmtAssign {
                    lhs_offset: offset,
                    lhs: ident,
                    assign_token,
                    rhs,
                    semi_token,
                })
            }

            // (ident "=") で始まらなかった場合のルート.
            // tokensは進んでいないことに注意。
            _ => Stmt::Expr(self.parse_expr(tokens)),
        }
    }

    /// > expr          = equality
    ///
    /// で表現される記号exprをパースする関数。
    pub fn parse_expr(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        self.parse_equality(tokens)
    }

    /// > equality      = relational ("==" relational | "!=" relational)*
    ///
    /// で表現される記号equalityをパースする関数。
    pub fn parse_equality(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        let mut expr = self.parse_relational(tokens);

        while let Some(token) = tokens.peek() {
            let op = match token {
                Token::Eq(eq_token) => BinOp::Eq(eq_token),
                Token::Neq(neq_token) => BinOp::Neq(neq_token),
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();

            let rhs = self.parse_relational(tokens);

            expr = Expr::BinOp(ExprBinOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            });
        }

        expr
    }

    /// > relational    = ("<" add | "<=" add | ">" add | ">=" add)*
    ///
    /// で表現される記号relationalをパースする関数。
    pub fn parse_relational(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        let mut expr = self.parse_add(tokens);

        while let Some(token) = tokens.peek() {
            let (op, reverse) = match token {
                Token::Lt(token) => (BinOp::Lt(token), false),
                Token::Lte(token) => (BinOp::Lte(token), false),
                Token::Gt(token) => (BinOp::Lt(Lt::new(token.pos)), true),
                Token::Gte(token) => (BinOp::Lte(Lte::new(token.pos)), true),
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();

            let another_expr = self.parse_add(tokens);

            let (lhs, rhs) = if reverse {
                (another_expr, expr)
            } else {
                (expr, another_expr)
            };

            expr = Expr::BinOp(ExprBinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });
        }

        expr
    }

    /// > add           = mul ("+" mul | "-" mul)*
    ///
    /// で表現される記号addをパースする関数。
    pub fn parse_add(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        let mut expr = self.parse_mul(tokens);

        while let Some(token) = tokens.peek() {
            let op = match token {
                Token::Add(token) => BinOp::Add(token),
                Token::Sub(token) => BinOp::Sub(token),
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();

            let rhs = self.parse_mul(tokens);

            expr = Expr::BinOp(ExprBinOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            });
        }

        expr
    }

    /// > mul       = unary ("*" unary | "/" unary)*
    ///
    /// で表現される記号mulをパースする関数。
    pub fn parse_mul(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        let mut expr = self.parse_unary(tokens);

        while let Some(token) = tokens.peek() {
            let op = match token {
                Token::Mul(token) => BinOp::Mul(token),
                Token::Div(token) => BinOp::Div(token),
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();

            let rhs = self.parse_unary(tokens);

            expr = Expr::BinOp(ExprBinOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            });
        }

        expr
    }

    /// > unary     = ("+" | "-")? primary
    ///
    /// で表現される記号unaryをパースする関数。
    pub fn parse_unary(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        match tokens.peek() {
            Some(Token::Add(token)) => {
                let _ = tokens.next();
                Expr::BinOp(ExprBinOp {
                    lhs: Box::new(Expr::Num(Num::new(0, token.pos))),
                    op: BinOp::Add(token),
                    rhs: Box::new(self.parse_primary(tokens)),
                })
            }
            Some(Token::Sub(token)) => {
                let _ = tokens.next();
                Expr::BinOp(ExprBinOp {
                    lhs: Box::new(Expr::Num(Num::new(0, token.pos))),
                    op: BinOp::Sub(token),
                    rhs: Box::new(self.parse_primary(tokens)),
                })
            }
            _ => self.parse_primary(tokens),
        }
    }

    /// > primary   = num | ident ( "(" ")" )? | "(" expr ")"
    ///
    /// で表現される記号primaryをパースする関数。
    pub fn parse_primary(&mut self, tokens: &mut TokenStream<'src>) -> Expr<'src> {
        match tokens.next() {
            Some(Token::Num(token)) => Expr::Num(token),
            Some(Token::Ident(ident)) => {
                match tokens.peek() {
                    // 関数呼び出しの場合
                    Some(Token::ParenLeft(paren_left_token)) => {
                        let _ = tokens.next();

                        let paren_right_token = parse_exact!(tokens, ParenRight);

                        Expr::Call(ExprCall {
                            ident,
                            paren_left_token,
                            paren_right_token,
                        })
                    }
                    _ => {
                        let offset = self.offset_of_local_var(ident.name);
                        Expr::Ident(ExprIdent {
                            ident_offset: offset,
                            ident,
                        })
                    }
                }
            }
            Some(Token::ParenLeft(paren_left_token)) => {
                let expr = self.parse_expr(tokens);
                let paren_right_token = parse_exact!(tokens, ParenRight);

                Expr::Paren(ExprParen {
                    paren_left_token,
                    expr: Box::new(expr),
                    paren_right_token,
                })
            }
            Some(token) => exit_with_err_msg(token.pos(), "expected number, ident or \"(\""),
            None => exit_with_err_msg(tokens.pos, "expected number, ident or \"(\""),
        }
    }
}

fn exit_with_err_msg<'src>(pos: Pos<'src>, msg: &str) -> ! {
    eprintln!("{}", pos.display(msg));
    std::process::exit(1)
}
