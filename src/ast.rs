use crate::token::{Op, Par, Token, TokenIter, TokenKind};
use std::collections::HashMap;

pub enum Node {
    Assign(AssignNode),
    Expr(ExprNode),
}

pub struct AssignNode {
    pub lhs_ident_offset: usize,
    pub rhs: ExprNode,
}

pub enum ExprNode {
    /// 末端Node
    Num(usize),
    Ident {
        offset: usize,
    },
    /// 非末端Node
    Op(OpNode),
}

pub struct OpNode {
    pub kind: Op,
    pub lhs: Box<ExprNode>,
    pub rhs: Box<ExprNode>,
}

impl ExprNode {
    /// 数値を表すNodeを作成する。
    pub fn new_num(i: usize) -> Self {
        ExprNode::Num(i)
    }

    /// 変数を表すNodeを作成する。
    pub fn new_ident(offset: usize) -> Self {
        ExprNode::Ident { offset }
    }

    /// 左辺と右辺を受け取る２項演算子を表すNodeを作成する
    pub fn new_op(op: Op, lhs: ExprNode, rhs: ExprNode) -> Self {
        ExprNode::Op(OpNode {
            kind: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

pub struct Parser<'a> {
    local_vars: HashMap<&'a str, usize>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            local_vars: HashMap::new(),
        }
    }

    /// > program       = stmt*
    /// > stmt          = assign ";"
    /// > assign        = (ident "=")? expr
    /// > expr          = equality
    /// > equality      = relational ("==" relational | "!=" relational)*
    /// > relational    = add ("<" add | "<=" add | ">" add | ">=" add)*
    /// > add           = mul ("+" mul | "-" mul)*
    /// > unary         = ("+" | "-")? primary
    /// > primary       = num | ident | "(" expr ")"
    ///
    /// で表現される文法をパースする関数。
    pub fn parse(&mut self, tokens: &mut TokenIter<'a>) -> Vec<Node> {
        self.parse_program(tokens)
    }

    /// > program       = stmt*
    ///
    /// で表現される記号programをパースする関数。
    pub fn parse_program(&mut self, tokens: &mut TokenIter<'a>) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(_) = tokens.peek() {
            nodes.push(self.parse_stmt(tokens))
        }
        nodes
    }

    /// > stmt          = assign ";"
    ///
    /// で表現される記号stmtをパースする関数。
    pub fn parse_stmt(&mut self, tokens: &mut TokenIter<'a>) -> Node {
        let node = self.parse_assign(tokens);
        match tokens.next() {
            Some(Token {
                kind: TokenKind::Semi,
                ..
            }) => node,
            Some(t) => t.exit_with_err_msg("expected \";\" but found another"),
            None => tokens.exit_with_err_msg("expected \";\" but found EOF"),
        }
    }

    /// > assign        = (ident "=")? expr
    ///
    /// で表現される記号assignをパースする関数。
    pub fn parse_assign(&mut self, tokens: &mut TokenIter<'a>) -> Node {
        // 与えられたTokenIterが (ident "=") で始まるかチェックする
        let mut tokens2 = *tokens;
        match (tokens2.next(), tokens2.next()) {
            (
                Some(Token {
                    kind: TokenKind::Ident(s),
                    ..
                }),
                Some(Token {
                    kind: TokenKind::Op(Op::Assign),
                    ..
                }),
            ) => {
                // (ident "=") で始まった場合のルート.
                // tokensを2つ進める。
                tokens.next();
                tokens.next();
                Node::Assign(AssignNode {
                    // TODO
                    lhs_ident_offset: 0,
                    rhs: self.parse_expr(tokens),
                })
            }
            // (ident "=") で始まらなかった場合のルート.
            // tokensは進んでいないことに注意。
            _ => Node::Expr(self.parse_expr(tokens)),
        }
    }

    /// > expr          = equality
    ///
    /// で表現される記号exprをパースする関数。
    pub fn parse_expr(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        self.parse_equality(tokens)
    }

    /// > equality      = relational ("==" relational | "!=" relational)*
    ///
    /// で表現される記号equalityをパースする関数。
    pub fn parse_equality(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        let mut node = self.parse_relational(tokens);
        while let Some(token) = tokens.peek() {
            let op = match token.op() {
                Some(op @ Op::Eq) => op,
                Some(op @ Op::Neq) => op,
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();
            let rhs = self.parse_relational(tokens);
            node = ExprNode::new_op(op, node, rhs);
        }
        node
    }

    /// > relational    = ("<" add | "<=" add | ">" add | ">=" add)*
    ///
    /// で表現される記号relationalをパースする関数。
    pub fn parse_relational(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        let mut node = self.parse_add(tokens);
        while let Some(token) = tokens.peek() {
            let (op, reverse) = match token.op() {
                Some(op @ Op::Lt) => (op, false),
                Some(op @ Op::Lte) => (op, false),
                Some(Op::Gt) => (Op::Lt, true),
                Some(Op::Gte) => (Op::Lte, true),
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();
            if reverse {
                node = ExprNode::new_op(op, self.parse_add(tokens), node);
            } else {
                node = ExprNode::new_op(op, node, self.parse_add(tokens));
            }
        }
        node
    }

    /// > add           = mul ("+" mul | "-" mul)*
    ///
    /// で表現される記号addをパースする関数。
    pub fn parse_add(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        let mut node = self.parse_mul(tokens);
        while let Some(token) = tokens.peek() {
            let op = match token.op() {
                Some(op @ Op::Add) => op,
                Some(op @ Op::Sub) => op,
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();
            let rhs = self.parse_mul(tokens);
            node = ExprNode::new_op(op, node, rhs);
        }
        node
    }

    /// > mul       = unary ("*" unary | "/" unary)*
    ///
    /// で表現される記号mulをパースする関数。
    pub fn parse_mul(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        let mut node = self.parse_unary(tokens);
        while let Some(token) = tokens.peek() {
            let op = match token.op() {
                Some(op @ Op::Mul) => op,
                Some(op @ Op::Div) => op,
                _ => break,
            };

            // このルートに入ることが確定したのでイテレータを進める
            let _ = tokens.next();
            node = ExprNode::new_op(op, node, self.parse_unary(tokens));
        }
        node
    }

    /// > unary     = ("+" | "-")? primary
    ///
    /// で表現される記号unaryをパースする関数。
    pub fn parse_unary(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        match tokens.peek().and_then(|token| token.op()) {
            Some(Op::Add) => {
                let _ = tokens.next();
                ExprNode::new_op(Op::Add, ExprNode::new_num(0), self.parse_primary(tokens))
            }
            Some(Op::Sub) => {
                let _ = tokens.next();
                ExprNode::new_op(Op::Sub, ExprNode::new_num(0), self.parse_primary(tokens))
            }
            _ => self.parse_primary(tokens),
        }
    }

    /// > primary   = num | ident | "(" expr ")"
    ///
    /// で表現される記号primaryをパースする関数。
    pub fn parse_primary(&mut self, tokens: &mut TokenIter<'a>) -> ExprNode {
        let token = tokens.next().unwrap_or_else(|| {
            tokens.exit_with_err_msg("Unexpected EOF. number, ident or \"(\" is expected")
        });

        if let Some(n) = token.num() {
            ExprNode::new_num(n)
        } else if let Some(c) = token.ident() {
            // TODO
            ExprNode::new_ident(0)
        } else {
            if !matches!(token.expect_par(), Par::Left) {
                token.exit_with_err_msg("expect \"(\" instead of \")\"");
            }

            let node = self.parse_expr(tokens);

            let token = tokens
                .next()
                .unwrap_or_else(|| tokens.exit_with_err_msg("Unexpected EOF. \")\" is expected"));
            if !matches!(token.expect_par(), Par::Right) {
                token.exit_with_err_msg("expect \")\" instead of \"(\"");
            }

            node
        }
    }
}
