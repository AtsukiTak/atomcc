use super::{addr::Address, reg::*, Asm};

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
    // 0 引数のinstruction
    (
        $lower:ident =>
        $(#[$outer:meta])*
        pub struct $ty:ident
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        $(#[$outer])*
        pub struct $ty();

        pub fn $lower() -> $ty {
            $ty()
        }

        impl $ty {
            pub const fn opcode() -> &'static str {
                stringify!($lower)
            }
        }
    };

    (
        $lower: ident =>
        $(#[$outer:meta])*
        pub struct $ty:ident<$t1: tt $(, $tn: tt)*>
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        $(#[$outer])*
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
    // 0 引数のinstruction
    ($ty:tt) => {
        impl Asm for $ty {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {}\n", Self::opcode())
            }
        }
    };

    // 1 引数のinstruction
    ($ty:tt<$t1:ty>) => {
        impl Asm for $ty<$t1> {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {} {}\n", Self::opcode(), self.0)
            }
        }
    };

    // 2 引数のinstruction
    ($ty:tt<$t1:ty, $t2:ty>) => {
        impl Asm for $ty<$t1, $t2> {
            fn write(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
                write!(w, "  {} {}, {}\n", Self::opcode(), self.0, self.1)
            }
        }
    };

    // 2 引数のinstruction (where句あり)
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
instruction! {cmp =>
    /// `T1` と `T2` を比較し、その結果をEFLAGSレジスタの
    /// ステータスフラグにセットする。
    /// 比較は `T1` から `T2` を引き, `sub` 命令と同じように
    /// ステータスフラグをセットする。
    /// ただし `sub` 命令と違い、 `T1` が更新されることはない。
    pub struct Cmp<T1, T2>
}
impl_asm!(Cmp<Reg64, i64>);
impl_asm!(Cmp<Reg64, Reg64>);

// mov
instruction! {mov =>
    /// `T2` の値を `T1` にコピーする
    /// `T1` = `T2`;
    pub struct Mov<T1, T2>
}
impl_asm!(Mov<Reg64, Reg64>);
impl_asm!(Mov<A, Reg64> where A: Address);
impl_asm!(Mov<Reg64, A> where A: Address);

// movzx
instruction! {movzx =>
    /// `T2` の値をゼロ拡張して `T1` にコピーする
    pub struct Movzx<T1, T2>
}
impl_asm!(Movzx<Reg64, Reg8>);

// pop
instruction! {pop =>
    /// スタックトップの値をpopし、`T` にコピーする
    pub struct Pop<T>
}
impl_asm!(Pop<Reg64>);

// push
instruction! {push =>
    /// `T` の値をスタックトップにpushする
    pub struct Push<T>
}
impl_asm!(Push<Reg64>);
impl_asm!(Push<i64>);

// ret
instruction! {ret =>
    pub struct Ret
}
impl_asm!(Ret);

// add
instruction! {add =>
    /// `T1` = `T1` + `T2`
    pub struct Add<T1, T2>
}
impl_asm!(Add<Reg64, Reg64>);

// sub
instruction! {sub =>
    /// `T1` = `T1` - `T2`
    pub struct Sub<T1, T2>
}
impl_asm!(Sub<Reg64, i64>);
impl_asm!(Sub<Reg64, Reg64>);

// imul
instruction! {imul =>
    pub struct Imul<T1, T2>
}
impl_asm!(Imul<Reg64, Reg64>);

// cqo
instruction! {cqo =>
    pub struct Cqo
}
impl_asm!(Cqo);

// idiv
instruction! {idiv =>
    pub struct Idiv<T>
}
impl_asm!(Idiv<Reg64>);

// sete
instruction! {sete =>
    /// ZF（ゼロフラグ）がセットされていれば（ZF == 1 であれば）
    /// 指定された場所に1を書き込む。
    /// セットされていなければ0を書き込む。
    pub struct Sete<T>
}
impl_asm!(Sete<Reg8>);

// setne
instruction! {setne =>
    /// ZF（ゼロフラグ）がセットされていなければ（ZF == 0 であれば）
    /// 指定された場所に1を書き込む。
    /// セットされていなければ0を書き込む。
    pub struct Setne<T>
}
impl_asm!(Setne<Reg8>);

// setl
instruction! {setl =>
    /// SF（符号フラグ）と OF（オーバーフローフラグ）が等しくなければ
    /// 指定された場所に1を書き込む。
    /// セットされていなければ0を書き込む。
    pub struct Setl<T>
}
impl_asm!(Setl<Reg8>);

// setle
instruction! {setle =>
    /// SF（符号フラグ）と OF（オーバーフローフラグ）が等しくなければ
    /// 指定された場所に1を書き込む。
    /// セットされていなければ0を書き込む。
    pub struct Setle<T>
}
impl_asm!(Setle<Reg8>);
