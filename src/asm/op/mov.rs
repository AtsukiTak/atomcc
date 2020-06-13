use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mov<S, D> {
    dst: D,
    src: S,
}

impl Mov<Reg64, Reg64> {
    pub fn new(dst: Reg64, src: Reg64) -> Self {
        Mov { dst, src }
    }
}

impl Instruction for Mov<Reg64, Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  mov {}, {}\n", self.dst, self.src)
    }
}
