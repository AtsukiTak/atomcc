pub mod addr;
pub mod instructions;
pub mod reg;

pub use addr::Addr;
pub use reg::{Reg16, Reg32, Reg64, Reg8};

pub trait Asm {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}

pub struct Arbitrary(String);

pub fn arbitraty(s: impl Into<String>) -> Arbitrary {
    Arbitrary(s.into())
}

impl Asm for Arbitrary {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "{}\n", self.0)
    }
}
