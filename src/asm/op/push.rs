use super::super::{Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Push<T>(pub T);

impl Instruction for Push<Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  push {}\n", self.0)
    }
}

impl Instruction for Push<i64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  push {}\n", self.0)
    }
}
