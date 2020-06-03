use crate::token::{Op, Par, Token, TokenIter, TokenKind};

pub enum Node {
    Assign(AssignNode),
    Expr(ExprNode),
}

pub struct AssignNode {
    pub lhs_ident: u8,
    pub rhs: ExprNode,
}

pub enum ExprNode {
    /// 末端Node
    Num(usize),
    Ident(u8),
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
    pub fn new_ident(i: u8) -> Self {
        ExprNode::Ident(i)
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
pub fn parse(tokens: &mut TokenIter) -> Vec<Node> {
    program(tokens)
}

/// > program       = stmt*
pub fn program(tokens: &mut TokenIter) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(_) = tokens.peek() {
        nodes.push(stmt(tokens))
    }
    nodes
}

/// > stmt          = assign ";"
pub fn stmt(tokens: &mut TokenIter) -> Node {
    let node = assign(tokens);
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
pub fn assign(tokens: &mut TokenIter) -> Node {
    // 与えられたTokenIterが (ident "=") で始まるかチェックする
    let mut tokens2 = *tokens;
    match (tokens2.next(), tokens2.next()) {
        // (ident "=") で始まった場合のルート.
        // tokensを2つ進める。
        (
            Some(Token {
                kind: TokenKind::Ident(c),
                ..
            }),
            Some(Token {
                kind: TokenKind::Op(Op::Assign),
                ..
            }),
        ) => {
            tokens.next();
            tokens.next();
            Node::Assign(AssignNode {
                lhs_ident: c,
                rhs: expr(tokens),
            })
        }
        // (ident "=") で始まらなかった場合のルート.
        // tokensは進んでいないことに注意。
        _ => Node::Expr(expr(tokens)),
    }
}

/// > expr          = equality
pub fn expr(tokens: &mut TokenIter) -> ExprNode {
    equality(tokens)
}

/// > equality      = relational ("==" relational | "!=" relational)*
pub fn equality(tokens: &mut TokenIter) -> ExprNode {
    let mut node = relational(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token.op() {
            Some(op @ Op::Eq) => op,
            Some(op @ Op::Neq) => op,
            _ => break,
        };

        // このルートに入ることが確定したのでイテレータを進める
        let _ = tokens.next();
        let rhs = relational(tokens);
        node = ExprNode::new_op(op, node, rhs);
    }
    node
}

/// > relational    = ("<" add | "<=" add | ">" add | ">=" add)*
pub fn relational(tokens: &mut TokenIter) -> ExprNode {
    let mut node = add(tokens);
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
            node = ExprNode::new_op(op, add(tokens), node);
        } else {
            node = ExprNode::new_op(op, node, add(tokens));
        }
    }
    node
}

/// > add           = mul ("+" mul | "-" mul)*
pub fn add(tokens: &mut TokenIter) -> ExprNode {
    let mut node = mul(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token.op() {
            Some(op @ Op::Add) => op,
            Some(op @ Op::Sub) => op,
            _ => break,
        };

        // このルートに入ることが確定したのでイテレータを進める
        let _ = tokens.next();
        let rhs = mul(tokens);
        node = ExprNode::new_op(op, node, rhs);
    }
    node
}

/// > mul       = unary ("*" unary | "/" unary)*
pub fn mul(tokens: &mut TokenIter) -> ExprNode {
    let mut node = unary(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token.op() {
            Some(op @ Op::Mul) => op,
            Some(op @ Op::Div) => op,
            _ => break,
        };

        // このルートに入ることが確定したのでイテレータを進める
        let _ = tokens.next();
        node = ExprNode::new_op(op, node, unary(tokens));
    }
    node
}

/// > unary     = ("+" | "-")? primary
///
/// で表現される非終端記号unaryをパースする関数。
pub fn unary(tokens: &mut TokenIter) -> ExprNode {
    match tokens.peek().and_then(|token| token.op()) {
        Some(Op::Add) => {
            let _ = tokens.next();
            ExprNode::new_op(Op::Add, ExprNode::new_num(0), primary(tokens))
        }
        Some(Op::Sub) => {
            let _ = tokens.next();
            ExprNode::new_op(Op::Sub, ExprNode::new_num(0), primary(tokens))
        }
        _ => primary(tokens),
    }
}

/// > primary   = num | ident | "(" expr ")"
///
/// で表現される非終端記号primaryをパースする関数。
pub fn primary(tokens: &mut TokenIter) -> ExprNode {
    let token = tokens.next().unwrap_or_else(|| {
        tokens.exit_with_err_msg("Unexpected EOF. number, ident or \"(\" is expected")
    });

    if let Some(n) = token.num() {
        ExprNode::new_num(n)
    } else if let Some(c) = token.ident() {
        ExprNode::new_ident(c)
    } else {
        if !matches!(token.expect_par(), Par::Left) {
            token.exit_with_err_msg("expect \"(\" instead of \")\"");
        }

        let node = expr(tokens);

        let token = tokens
            .next()
            .unwrap_or_else(|| tokens.exit_with_err_msg("Unexpected EOF. \")\" is expected"));
        if !matches!(token.expect_par(), Par::Right) {
            token.exit_with_err_msg("expect \")\" instead of \"(\"");
        }

        node
    }
}
