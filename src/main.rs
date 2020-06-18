pub mod asm;
pub mod generator;
pub mod parser;
pub mod token;
pub mod tokenizer;

use asm::{arbitrary, instructions::*, AsmBuf, Reg64::*};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = tokenizer::tokenize(arg.as_str());

    let nodes = parser::Parser::new().parse(&mut token_iter);

    let mut asm = AsmBuf::new();

    asm.push(arbitrary(".intel_syntax noprefix"));
    asm.push(arbitrary(".global _main"));
    asm.push(arbitrary("_main:"));

    // プロローグ
    // 変数26個分の領域を確保する
    asm.push(push(RBP));
    asm.push(mov(RBP, RSP));
    asm.push(sub(RSP, 8 * 26));

    let mut generator = generator::Generator::new();
    for node in nodes {
        generator.gen(&node, &mut asm);
    }

    asm.push(pop(RAX));

    // エピローグ
    asm.push(mov(RSP, RBP));
    asm.push(pop(RBP));
    asm.push(arbitrary("  ret"));

    asm.output_stdout().unwrap();
}
