use super::{addr::Address, reg::Reg64, Asm};

/// Instructionを表す構造体を定義する
///
/// ```
/// pub struct Mov<T1, T2>(pub T1, pub T2);
///
/// pub fn mov<T1, T2>(T1: T1, T2, T2) -> Mov<T1, T2> {
///     Mov(T1, T2)
/// }
///
/// impl<T1, T2> Mov<T1, T2> {
///     pub const fn opcode() -> &'static str {
///         "mov"
///     }
/// }
/// ```
macro_rules! instruction {
    ($lower: tt, $ty: tt<$t1: tt $(, $tn: tt)*>) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $ty<$t1 $(, $tn)*>(pub $t1 $(, pub $tn)*);

        #[allow(non_snake_case)]
        pub fn $lower<$t1 $(, $tn)*>($t1: $t1 $(, $tn: $tn)*) -> $ty<$t1 $(, $tn)*> {
            $ty($t1 $(, $tn)*)
        }

        impl<$t1 $(, $tn)*> $ty<$t1 $(, $tn)*> {
            pub const fn opcode() -> &'static str {
                stringify!($lower)
            }
        }
    };
}

/// `Asm` trait を実装する
///
/// ```
/// impl Asm for Mov<Reg64, Reg64> {
///     fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
///         write!(w, "  {} {}, {}\n", Self::opcode(), self.0, self.1)
///     }
/// }
/// ```
macro_rules! impl_asm {
    ($ty:tt<$t1:ty>) => {
        impl Asm for $ty<$t1> {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {} {}\n", Self::opcode(), self.0)
            }
        }
    };

    ($ty:tt<$t1:ty, $t2:ty>) => {
        impl Asm for $ty<$t1, $t2> {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {} {}, {}\n", Self::opcode(), self.0, self.1)
            }
        }
    };

    ($ty:tt<$t1:ty, $t2:ty> where A: Address) => {
        impl<A> Asm for $ty<$t1, $t2>
        where
            A: Address,
        {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {} {}, {}\n", Self::opcode(), self.0, self.1)
            }
        }
    };
}

// cmp
instruction!(cmp, Cmp<T1, T2>);
impl_asm!(Cmp<Reg64, i64>);
impl_asm!(Cmp<Reg64, Reg64>);

// mov
instruction!(mov, Mov<T1, T2>);
impl_asm!(Mov<Reg64, Reg64>);
impl_asm!(Mov<A, Reg64> where A: Address);
impl_asm!(Mov<Reg64, A> where A: Address);

// pop
instruction!(pop, Pop<T>);
impl_asm!(Pop<Reg64>);

// push
instruction!(push, Push<T>);
impl_asm!(Push<Reg64>);
impl_asm!(Push<i64>);

// sub
instruction!(sub, Sub<T1, T2>);
impl_asm!(Sub<Reg64, i64>);
impl_asm!(Sub<Reg64, Reg64>);
