use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Push<S> {
    src: S,
}

impl<S> Push<S> {
    pub fn new(src: S) -> Self {
        Push { src }
    }
}

impl Instruction for Push<Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  push {}\n", self.src)
    }
}

impl Instruction for Push<i64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  push {}\n", self.src)
    }
}
