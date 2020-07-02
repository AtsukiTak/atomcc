pub mod addr;
mod buf;
pub mod instructions;
pub mod reg;

pub use addr::Addr;
pub use buf::AsmBuf;
pub use reg::{Reg16, Reg32, Reg64, Reg8};

use std::io::{Result as IoResult, Write};

pub trait Asm {
    fn write(&self, w: &mut dyn Write) -> IoResult<()>;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}

pub struct Arbitrary(String);

pub fn arbitrary(s: impl Into<String>) -> Arbitrary {
    Arbitrary(s.into())
}

impl Asm for Arbitrary {
    fn write(&self, w: &mut dyn Write) -> IoResult<()> {
        write!(w, "{}\n", self.0)
    }
}
