use crate::token::{Op, Par, TokenIter};

pub enum Node {
    /// 末端Node
    Num(usize),
    /// 非末端Node
    Op(OpNode),
}

pub struct OpNode {
    pub kind: Op,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

impl Node {
    /// 数値を表すNodeを作成する。
    pub fn new_num(i: usize) -> Node {
        Node::Num(i)
    }

    /// 左辺と右辺を受け取る２項演算子を表すNodeを作成する
    pub fn new_op(op: Op, lhs: Node, rhs: Node) -> Node {
        Node::Op(OpNode {
            kind: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

/// > expr = mul ("+" mul | "-" mul)*
///
/// で表現される非終端記号exprをパースする関数。
pub fn expr(tokens: &mut TokenIter) -> Node {
    let mut node = mul(tokens);
    while let Some(token) = tokens.next() {
        match token.expect_op() {
            op @ Op::Add => node = Node::new_op(op, node, mul(tokens)),
            op @ Op::Sub => node = Node::new_op(op, node, mul(tokens)),
            _ => token.exit_with_err_msg("expect \"+\" or \"-\""),
        }
    }
    node
}

/// > mul = primary ("*" primary | "/" primary)*
///
/// で表現される非終端記号mulをパースする関数。
pub fn mul(tokens: &mut TokenIter) -> Node {
    let mut node = primary(tokens);
    while let Some(token) = tokens.next() {
        match token.expect_op() {
            op @ Op::Mul => node = Node::new_op(op, node, primary(tokens)),
            op @ Op::Div => node = Node::new_op(op, node, primary(tokens)),
            _ => token.exit_with_err_msg("expect \"*\" or \"/\""),
        }
    }
    node
}

/// > primary = num | "(" expr ")"
///
/// で表現される非終端記号primaryをパースする関数。
pub fn primary(tokens: &mut TokenIter) -> Node {
    let token = tokens.next().unwrap_or_else(|| {
        tokens.exit_with_err_msg("Unexpected EOF. number, \"(\" or \")\" is expected")
    });

    if token.is_num() {
        Node::new_num(token.expect_num())
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
