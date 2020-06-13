pub mod asm;
pub mod generator;
pub mod parser;
pub mod token;

use asm::{op, Instruction as _, Reg64::*};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = token::tokenize(arg.as_str());
    let nodes = parser::Parser::new().parse(&mut token_iter);

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // プロローグ
    // 変数26個分の領域を確保する
    op::Push::new(RBP).print();
    op::Mov::new(RBP, RSP).print();
    op::Sub::new(RSP, 8 * 26);

    for node in nodes {
        generator::gen(&node);
    }

    println!("  pop rax");

    // エピローグ
    op::Mov::new(RSP, RBP).print();
    println!("  pop rbp");
    println!("  ret");
}
