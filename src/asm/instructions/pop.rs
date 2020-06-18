use super::super::{reg::Reg64, Asm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pop<T>(pub T);

impl Asm for Pop<Reg64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()>
    {
        write!(w, "  pop {}\n", self.0)
    }
}
