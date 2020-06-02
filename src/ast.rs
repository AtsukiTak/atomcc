use crate::token::{Op, Par, Token, TokenIter, TokenKind};

pub enum Node {
    /// 末端Node
    Num(usize),
    /// 末端Node
    Ident(char),
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

    /// 変数を表すNodeを作成する。
    pub fn new_ident(i: char) -> Node {
        Node::Ident(i)
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

// > stmt*
pub fn program(tokens: &mut TokenIter) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(_) = tokens.peek() {
        nodes.push(stmt(tokens))
    }
    nodes
}

/// > stmt = expr ";"
pub fn stmt(tokens: &mut TokenIter) -> Node {
    let node = expr(tokens);
    match tokens.next() {
        Some(Token {
            kind: TokenKind::Semi,
            ..
        }) => node,
        Some(t) => t.exit_with_err_msg("expected \";\" but found another"),
        None => tokens.exit_with_err_msg("expected \";\" but found EOF"),
    }
}

/// > expr = assign
pub fn expr(tokens: &mut TokenIter) -> Node {
    equality(tokens)
}

/// > assign = equality ("=" assign)?
pub fn assign(tokens: &mut TokenIter) -> Node {
    let node = equality(tokens);
    match tokens.peek() {
        Some(Token {
            kind: TokenKind::Op(Op::Assign),
            ..
        }) => Node::new_op(Op::Assign, node, assign(tokens)),
        _ => node,
    }
}

/// > equality = relational ("==" relational | "!=" relational)*
///
/// で表現される非終端義号equalityをパースする関数
pub fn equality(tokens: &mut TokenIter) -> Node {
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
        node = Node::new_op(op, node, rhs);
    }
    node
}

/// > add = ("<" add | "<=" add | ">" add | ">=" add)*
///
/// で表現される非終端記号relationalをパースする関数
pub fn relational(tokens: &mut TokenIter) -> Node {
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
            node = Node::new_op(op, add(tokens), node);
        } else {
            node = Node::new_op(op, node, add(tokens));
        }
    }
    node
}

/// > add = mul ("+" mul | "-" mul)*
///
/// で表される非終端記号addをパースする関数
pub fn add(tokens: &mut TokenIter) -> Node {
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
        node = Node::new_op(op, node, rhs);
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

/// > primary = num | ident | "(" expr ")"
///
/// で表現される非終端記号primaryをパースする関数。
pub fn primary(tokens: &mut TokenIter) -> Node {
    let token = tokens.next().unwrap_or_else(|| {
        tokens.exit_with_err_msg("Unexpected EOF. number, ident or \"(\" is expected")
    });

    if let Some(n) = token.num() {
        Node::new_num(n)
    } else if let Some(c) = token.ident() {
        Node::new_ident(c)
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
