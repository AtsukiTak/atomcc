use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Push<D> {
    dst: D,
}

impl Push<Reg64> {
    pub fn new(dst: Reg64) -> Self {
        Push { dst }
    }
}

impl Instruction for Push<Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  push {}", self.dst)
    }
}
