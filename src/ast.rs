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
    while let Some(token) = tokens.peek() {
        let op = match token.op() {
            Some(op @ Op::Add) => op,
            Some(op @ Op::Sub) => op,
            _ => break,
        };

        // このルートに入ることが確定したのでイテレータを進める
        let _ = tokens.next();
        node = Node::new_op(op, node, mul(tokens));
    }
    node
}

/// > mul = unary ("*" unary | "/" unary)*
///
/// で表現される非終端記号mulをパースする関数。
pub fn mul(tokens: &mut TokenIter) -> Node {
    let mut node = unary(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token.op() {
            Some(op @ Op::Mul) => op,
            Some(op @ Op::Div) => op,
            _ => break,
        };

        // このルートに入ることが確定したのでイテレータを進める
        let _ = tokens.next();
        node = Node::new_op(op, node, unary(tokens));
    }
    node
}

/// > unary = ("+" | "-")? primary
///
/// で表現される非終端記号unaryをパースする関数。
pub fn unary(tokens: &mut TokenIter) -> Node {
    match tokens.peek().and_then(|token| token.op()) {
        Some(Op::Add) => {
            let _ = tokens.next();
            Node::new_op(Op::Add, Node::new_num(0), primary(tokens))
        }
        Some(Op::Sub) => {
            let _ = tokens.next();
            Node::new_op(Op::Sub, Node::new_num(0), primary(tokens))
        }
        _ => primary(tokens),
    }
}

/// > primary = num | "(" expr ")"
///
/// で表現される非終端記号primaryをパースする関数。
pub fn primary(tokens: &mut TokenIter) -> Node {
    let token = tokens.next().unwrap_or_else(|| {
        tokens.exit_with_err_msg("Unexpected EOF. number, \"(\" or \")\" is expected")
    });

    if let Some(n) = token.num() {
        Node::new_num(n)
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
