use atomcc::{asm::AsmBuf, generator, parser, token::tokenize};

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    let mut token_iter = tokenize(arg.as_str());

    let nodes = parser::Parser::new().parse(&mut token_iter);

    let mut asm = AsmBuf::new();
    let mut generator = generator::Generator::new();
    generator.gen(&nodes, &mut asm);

    asm.output_stdout().unwrap();
}
