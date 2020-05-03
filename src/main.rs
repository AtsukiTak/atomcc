fn main() {
    let i = {
        let s = std::env::args().nth(1).unwrap();
        usize::from_str_radix(s.as_str(), 10).unwrap()
    };

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    println!("  mov rax, {}", i);
    println!("  ret");
}
