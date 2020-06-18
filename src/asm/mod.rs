pub mod addr;
pub mod instructions;
pub mod reg;

pub use addr::Addr;
pub use reg::{Reg16, Reg32, Reg64, Reg8};

use std::{
    fs::OpenOptions,
    io::{Result as IoResult, Write},
    path::Path,
};

pub trait Asm {
    fn write(&self, w: &mut dyn Write) -> IoResult<()>;

    fn print(&self) {
        self.write(&mut std::io::stdout()).unwrap()
    }
}

pub struct AsmBuf {
    vec: Vec<Box<dyn Asm>>,
}

impl AsmBuf {
    pub fn new() -> Self {
        AsmBuf { vec: Vec::new() }
    }

    pub fn push(&mut self, asm: impl Asm + 'static) {
        self.vec.push(Box::new(asm))
    }

    pub fn append(&mut self, others: &mut AsmBuf) {
        self.vec.append(&mut others.vec)
    }

    pub fn output<W>(&self, w: &mut W) -> IoResult<()>
    where
        W: Write,
    {
        for asm in self.vec.iter() {
            asm.write(w)?;
        }

        Ok(())
    }

    pub fn output_stdout(&self) -> IoResult<()> {
        self.output(&mut std::io::stdout())
    }

    pub fn output_file(&self, path: impl AsRef<Path>) -> IoResult<()> {
        let mut file = OpenOptions::new().create(true).write(true).open(path)?;
        self.output(&mut file)
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
