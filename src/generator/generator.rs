use super::subroutine::SubroutineGen;
use crate::{
    asm::{arbitrary, AsmBuf},
    parser::ast::*,
};

pub struct Generator();

impl Generator {
    pub fn new() -> Self {
        Generator()
    }

    pub fn gen<'a>(&mut self, stmts: &[Stmt<'a>], buf: &mut AsmBuf) {
        self.gen_prelude(buf);
        SubroutineGen::new().gen_subroutine(stmts, buf);
    }

    pub fn gen_prelude(&self, buf: &mut AsmBuf) {
        *buf += arbitrary(".intel_syntax noprefix");
        *buf += arbitrary(".global _main");
        *buf += arbitrary("_main:");
    }
}
