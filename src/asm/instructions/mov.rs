use super::super::{addr::Address, reg::Reg64, Asm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mov<T1, T2>(pub T1, pub T2);

pub fn mov<T1, T2>(t1: T1, t2: T2) -> Mov<T1, T2> {
    Mov(t1, t2)
}

impl Asm for Mov<Reg64, Reg64> {
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}

impl<A> Asm for Mov<Reg64, A>
where
    A: Address,
{
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}

impl<A> Asm for Mov<A, Reg64>
where
    A: Address,
{
    fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}
