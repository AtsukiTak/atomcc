pub mod ast;
pub mod generator;
pub mod token;

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = token::tokenize(arg.as_str());
    let ast = ast::expr(&mut token_iter);

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    generator::gen(&ast);

    println!("  pop rax");
    println!("  ret");
}
