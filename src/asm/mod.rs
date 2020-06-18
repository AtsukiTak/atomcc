pub mod addr;
pub mod instructions;
pub mod reg;

pub use addr::Addr;
pub use reg::{Reg16, Reg32, Reg64, Reg8};

/// This represents a single line of assembly code.
pub enum Asm {
    Arbitrary(String),
}

pub fn arbitraty(s: impl Into<String>) -> Asm {
    Asm::Arbitrary(s.into())
}

pub trait Instruction {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}
