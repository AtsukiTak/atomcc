use crate::{
    ast::{Node, OpNode},
    token::Op,
};

pub fn gen(node: &Node) {
    match node {
        Node::Num(n) => println!("  push {}", n),
        Node::Ident(c) => todo!(),
        Node::Op(OpNode { kind, lhs, rhs }) => {
            gen(lhs); // スタックトップに1つ値が残る（ようなコードを生成する）
            gen(rhs); // スタックトップに1つ値が残る（ようなコードを生成する）

            println!("  pop rdi"); // 左ブランチの計算結果をrdiレジスタに記録
            println!("  pop rax"); // 右ブランチの計算結果をraxレジスタに記録

            match kind {
                Op::Add => println!("  add rax, rdi"),
                Op::Sub => println!("  sub rax, rdi"),
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

            println!("  push rax");
        }
    }
}
