pub mod asm;
pub mod generator;
pub mod parser;
pub mod token;

use asm::{op::*, Instruction as _, Reg64::*};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = token::tokenize(arg.as_str());
    let nodes = parser::Parser::new().parse(&mut token_iter);

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // プロローグ
    // 変数26個分の領域を確保する
    Push::new(RBP).print();
    Mov::new(RBP, RSP).print();
    Sub::new(RSP, 8 * 26).print();

    for node in nodes {
        generator::gen(&node);
    }

    Pop::new(RAX).print();

    // エピローグ
    Mov::new(RSP, RBP).print();
    Pop::new(RBP).print();
    println!("  ret");
}
