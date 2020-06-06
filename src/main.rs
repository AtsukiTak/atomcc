pub mod generator;
pub mod parser;
pub mod token;

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = token::tokenize(arg.as_str());
    let nodes = parser::Parser::new().parse(&mut token_iter);

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", 8 * 26);

    for node in nodes {
        generator::gen(&node);
    }

    println!("  pop rax");

    // エピローグ
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
