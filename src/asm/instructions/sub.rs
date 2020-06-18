use super::super::{reg::Reg64, Asm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub<T1, T2>(pub T1, pub T2);

pub fn sub<T1, T2>(t1: T1, t2: T2) -> Sub<T1, T2> {
    Sub(t1, t2)
}

impl Asm for Sub<Reg64, i64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  sub {}, {}\n", self.0, self.1)
    }
}

impl Asm for Sub<Reg64, Reg64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  sub {}, {}\n", self.0, self.1)
    }
}
