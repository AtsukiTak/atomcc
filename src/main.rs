fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut expr = arg.split(" ");

    let n = {
        let s = expr.next().unwrap();
        usize::from_str_radix(s, 10).unwrap()
    };

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    println!("  mov rax, {}", n);

    while let Some(op) = expr.next() {
        let n = {
            let s = expr.next().unwrap();
            usize::from_str_radix(s, 10).unwrap()
        };

        match op {
            "+" => println!("  add rax, {}", n),
            "-" => println!("  sub rax, {}", n),
            _ => panic!("Unexpected Operator"),
        }
    }

    println!("  ret");
}
