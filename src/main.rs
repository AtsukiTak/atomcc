pub mod tokenizer;

use tokenizer::{tokenize, Op};

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut token_iter = tokenize(arg.as_str());

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    println!("  mov rax, {}", token_iter.next().unwrap().expect_num());

    while let Some(token) = token_iter.next() {
        let n = token_iter.next().unwrap().expect_num();
        match token.expect_op() {
            Op::Add => println!("  add rax, {}", n),
            Op::Sub => println!("  sub rax, {}", n),
            _ => panic!("unsupported"),
        }
    }

    println!("  ret");
}
