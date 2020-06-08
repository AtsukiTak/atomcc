use super::Instruction;
use super::Reg64;

pub struct Mov<S, D> {
    src: S,
    dst: D,
}

impl Mov<Reg64, Reg64> {
    pub fn new(src: Reg64, dst: Reg64) -> Self {
        Mov { src, dst }
    }
}

impl Instruction for Mov<Reg64, Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  mov {}, {}", self.dst, self.src)
    }
}
