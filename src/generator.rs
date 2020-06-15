use crate::{
    asm::{op::*, reg::Reg64::*, Instruction as _},
    parser::{AssignNode, ExprNode, Node, OpNode},
    token::Op,
};

/// １つのstmtを処理するようなコードを生成する
pub fn gen(node: &Node) {
    match node {
        Node::Expr(expr) => gen_expr(expr),

        // ローカル変数にスタックトップの値を代入する
        Node::Assign(AssignNode {
            lhs_ident_offset,
            rhs,
        }) => {
            gen_expr(rhs);
            Pop(RAX).print();
            println!("  mov [rbp - {}], rax", lhs_ident_offset);
        }

        Node::Return(expr) => {
            gen_expr(expr);
            Pop(RAX).print();

            // エピローグ
            Mov(RSP, RBP).print();
            Pop(RBP).print();
            println!("  ret");
        }
    }
}

// スタックトップにexprの結果の値を1つ載せるようなコードを生成する
pub fn gen_expr(node: &ExprNode) {
    match node {
        // スタックトップに即値を載せる
        ExprNode::Num(n) => Push(*n as i64).print(),

        // スタックトップに変数の値を載せる
        ExprNode::Ident { offset } => {
            println!("  mov rax, [rbp - {}]", offset);
            Push(RAX).print();
        }

        // スタックトップに計算結果を載せる
        ExprNode::Op(OpNode { kind, lhs, rhs }) => {
            gen_expr(lhs); // スタックトップに1つ値が残る（ようなコードを生成する）
            gen_expr(rhs); // スタックトップに1つ値が残る（ようなコードを生成する）

            Pop(RDI).print(); // 左ブランチの計算結果をrdiレジスタに記録
            Pop(RAX).print(); // 右ブランチの計算結果をraxレジスタに記録

            match kind {
                Op::Add => println!("  add rax, rdi"),
                Op::Sub => Sub(RAX, RDI).print(),
                Op::Mul => println!("  imul rax, rdi"),
                Op::Div => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                Op::Eq => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzx rax, al");
                }
                Op::Neq => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzx rax, al");
                }
                Op::Lt => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzx rax, al");
                }
                Op::Lte => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzx rax, al");
                }
                _ => unreachable!(),
            }

            Push(RAX).print();
        }
    }
}
