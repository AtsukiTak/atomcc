use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub<D, S> {
    dst: D,
    src: S,
}

impl<D, S> Sub<D, S> {
    pub fn new(dst: D, src: S) -> Self {
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
