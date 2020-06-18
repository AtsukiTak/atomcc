use super::super::{reg::Reg64, Asm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Push<T>(pub T);

impl Asm for Push<Reg64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  push {}\n", self.0)
    }
}

impl Asm for Push<i64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  push {}\n", self.0)
    }
}
