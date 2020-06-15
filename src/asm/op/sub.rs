use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub<T1, T2>(pub T1, pub T2);

impl Instruction for Sub<Reg64, i64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  sub {}, {}\n", self.0, self.1)
    }
}

impl Instruction for Sub<Reg64, Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  sub {}, {}\n", self.0, self.1)
    }
}
