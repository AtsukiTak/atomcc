use super::Asm;
use std::{
    fs::OpenOptions,
    io::{Result as IoResult, Write},
    ops::AddAssign,
    path::Path,
};

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

impl<T> AddAssign<T> for AsmBuf
where
    T: Asm + 'static,
{
    fn add_assign(&mut self, asm: T) {
        self.vec.push(Box::new(asm))
    }
}
