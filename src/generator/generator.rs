use super::subroutine::SubroutineGen;
use crate::{
    asm::{arbitrary, AsmBuf},
    parser::ast::*,
};

pub struct Generator {
    next_label_num: usize,
}

impl Generator {
    pub fn new() -> Self {
        Generator { next_label_num: 0 }
    }

    pub fn new_label_num(&mut self) -> usize {
        let n = self.next_label_num;
        self.next_label_num += 1;
        n
    }

    pub fn gen<'a>(&mut self, stmts: &[Stmt<'a>], buf: &mut AsmBuf) {
        self.gen_prelude(buf);
        SubroutineGen::new(self).gen_subroutine(stmts, buf);
    }

    pub fn gen_prelude(&self, buf: &mut AsmBuf) {
        *buf += arbitrary(".intel_syntax noprefix");
        *buf += arbitrary(".global _main");
        *buf += arbitrary("_main:");
    }
}
