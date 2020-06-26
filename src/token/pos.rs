use std::ops::{Add, AddAssign};

/// オリジナルのソースコード上のある位置を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos<'origin> {
    pub origin: &'origin str,
    pub pos: usize,
}

impl<'origin> Pos<'origin> {
    pub fn new(origin: &'origin str) -> Self {
        Pos { origin, pos: 0 }
    }

    /// ```text
    /// let i = return;
    ///         ^ unexpected token "return"
    /// ```
    ///
    /// のような文字列を返す
    pub fn display(&self, msg: &str) -> String {
        // TODO
        // 複数行のソースコードに対応
        let leading_spaces = " ".repeat(self.pos);
        format!("{}\n{}^ {}", self.origin, leading_spaces, msg)
    }
}

/// 位置を動かす
impl<'origin> Add<usize> for Pos<'origin> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        Pos {
            origin: self.origin,
            pos: self.pos + rhs,
        }
    }
}

/// 位置を動かす
impl<'origin> AddAssign<usize> for Pos<'origin> {
    fn add_assign(&mut self, rhs: usize) {
        self.pos += rhs;
    }
}
