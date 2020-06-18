use super::reg::Reg64;
use std::fmt::{Display, Formatter, Result};

pub struct Addr<T>(pub T);

pub trait Address: Display {}

/// Represents "T1 - T2"
pub struct Sub<T1, T2>(T1, T2);

/*
 * Addr<Reg64>
 */
impl Display for Addr<Reg64> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}]", self.0)
    }
}

impl Address for Addr<Reg64> {}

impl std::ops::Sub<i64> for Addr<Reg64> {
    type Output = Addr<Sub<Reg64, i64>>;

    fn sub(self, rhs: i64) -> Self::Output {
        Addr(Sub(self.0, rhs))
    }
}

/*
 * Addr<Sub<Reg64, i64>>
 */
impl Display for Addr<Sub<Reg64, i64>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{} - {}]", (self.0).0, (self.0).1)
    }
}

impl Address for Addr<Sub<Reg64, i64>> {}
