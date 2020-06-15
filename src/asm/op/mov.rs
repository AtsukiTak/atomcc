use super::super::{addr::Address, Instruction, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mov<T1, T2>(pub T1, pub T2);

impl Instruction for Mov<Reg64, Reg64> {
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}

impl<A> Instruction for Mov<Reg64, A>
where
    A: Address,
{
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}

impl<A> Instruction for Mov<A, Reg64>
where
    A: Address,
{
    fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(w, "  mov {}, {}\n", self.0, self.1)
    }
}
