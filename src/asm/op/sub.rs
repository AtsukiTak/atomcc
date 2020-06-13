use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub<D, S> {
    dst: D,
    src: S,
}

impl Sub<Reg64, i64> {
    pub fn new(dst: Reg64, src: i64) -> Self {
        Sub { dst, src }
    }
}

impl Instruction for Sub<Reg64, i64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  sub {}, {}\n", self.dst, self.src)
    }
}
