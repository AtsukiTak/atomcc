use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pop<D> {
    dst: D,
}

impl Pop<Reg64> {
    pub fn new(dst: Reg64) -> Self {
        Pop { dst }
    }
}

impl Instruction for Pop<Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  pop {}\n", self.dst)
    }
}
