pub mod asm;
pub mod generator;
pub mod parser;
pub mod token;
pub mod tokenizer;

use asm::{instructions::*, Instruction as _, Reg64::*};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = tokenizer::tokenize(arg.as_str());

    let nodes = parser::Parser::new().parse(&mut token_iter);

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // プロローグ
    // 変数26個分の領域を確保する
    Push(RBP).print();
    Mov(RBP, RSP).print();
    Sub(RSP, 8 * 26).print();

    let mut generator = generator::Generator::new();
    for node in nodes {
        generator.gen(&node);
    }

    Pop(RAX).print();

    // エピローグ
    Mov(RSP, RBP).print();
    Pop(RBP).print();
    println!("  ret");
}
