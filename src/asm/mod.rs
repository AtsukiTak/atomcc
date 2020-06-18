pub mod addr;
pub mod instructions;
pub mod reg;

pub use addr::Addr;
pub use reg::{Reg16, Reg32, Reg64, Reg8};

pub trait Asm {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()>;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}

pub struct AsmVec {
    vec: Vec<Box<dyn Asm>>,
}

impl AsmVec {
    pub fn new() -> Self {
        AsmVec { vec: Vec::new() }
    }

    pub fn push(&mut self, asm: impl Asm + 'static) {
        self.vec.push(Box::new(asm))
    }
}

pub struct Arbitrary(String);

pub fn arbitraty(s: impl Into<String>) -> Arbitrary {
    Arbitrary(s.into())
}

impl Asm for Arbitrary {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}\n", self.0)
    }
}
