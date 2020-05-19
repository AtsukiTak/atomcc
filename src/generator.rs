use crate::{
    ast::{Node, OpNode},
    token::Op,
};

pub fn gen(node: &Node) {
    match node {
        Node::Num(n) => println!("  push {}", n),
        Node::Op(OpNode { kind, lhs, rhs }) => {
            gen(lhs); // スタックトップに1つ値が残る（ようなコードを生成する）
            gen(rhs); // スタックトップに1つ値が残る（ようなコードを生成する）

            println!("  pop rdi"); // 左ブランチの計算結果をridレジスタに記録
            println!("  pop rax"); // 右ブランチの計算結果をraxレジスタに記録

            match kind {
                Op::Add => println!("  add rax, rdi"),
                Op::Sub => println!("  sub rax, rdi"),
                Op::Mul => println!("  imul rax, rdi"),
                Op::Div => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                _ => unreachable!(),
            }

            println!("  push rax");
        }
    }
}
