use super::Generator;
use crate::{
    asm::{arbitrary, instructions::*, Addr, AsmBuf, Reg64::*, Reg8::*},
    parser::ast::*,
};

/// サブルーチンのコードを生成するジェネレータ
pub struct SubroutineGen<'root> {
    root_gen: &'root mut Generator,
    /// `call` によって積まれるreturn addressも **含めた** スタックの長さ.
    /// 16 byte alignするときに使う。
    stack_len: usize,
}

impl<'root> SubroutineGen<'root> {
    pub fn new(root_gen: &'root mut Generator) -> Self {
        SubroutineGen {
            root_gen,
            stack_len: 0,
        }
    }

    pub fn inc_stack_len(&mut self) {
        self.stack_len += 8;
    }

    pub fn dec_stack_len(&mut self) {
        self.stack_len -= 8;
    }

    pub fn gen_subroutine<'a>(mut self, stmts: &[Stmt<'a>], buf: &mut AsmBuf) {
        self.gen_prologue(26, buf);

        for stmt in stmts {
            self.gen_stmt(stmt, buf);
        }

        // 最後にスタックに残っていた値をRAXレジスタにpopする。
        // C言語のABIでは返り値はRAXレジスタに入れる。
        // もうstack_lenの値は使わないのでdec_stack_lenしない
        buf.push(pop(RAX));

        self.gen_epilogue(buf);
    }

    // プロローグコードを修正
    // サブルーチンに移行するたびに呼び出す
    pub fn gen_prologue(&mut self, stack_bytes: i64, buf: &mut AsmBuf) {
        // return addressの分
        self.inc_stack_len();

        // ベースポインタの値を避難
        buf.push(push(RBP));
        self.inc_stack_len();

        // ベースポインタを、スタックポインタまで移動
        buf.push(mov(RBP, RSP));

        // stack領域の確保 (スタックポインタの移動)
        buf.push(sub(RSP, 8 * stack_bytes));
    }

    /// エピローグコードを生成
    /// サブルーチンから抜け出すたびに呼び出す
    ///
    /// エピローグを生成した後は同じSubroutineGen構造体を
    /// 使いまわせない。
    pub fn gen_epilogue(&mut self, buf: &mut AsmBuf) {
        // スタックポインタをベースポインタまで移動
        // ローカルスタック領域の開放
        buf.push(mov(RSP, RBP));

        // prologueで避難させておいたベースポインタの値を戻す
        // もうstack_lenの値は使わないので、dec_stack_lenしない
        buf.push(pop(RBP));

        // stackからreturn addressをpopし、そこにjumpする
        buf.push(ret());
    }

    /// １つのstmtを処理するようなコードを生成する
    pub fn gen_stmt<'a>(&mut self, stmt: &Stmt<'a>, buf: &mut AsmBuf) {
        match stmt {
            Stmt::Expr(expr) => self.gen_expr(expr, buf),

            // ローカル変数にスタックトップの値を代入する
            Stmt::Assign(StmtAssign {
                lhs_offset, rhs, ..
            }) => {
                self.gen_expr(rhs, buf);

                buf.push(pop(RAX));
                self.dec_stack_len();

                buf.push(mov(Addr(RBP) - *lhs_offset as i64, RAX));
            }

            Stmt::Return(StmtReturn { expr, .. }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr, buf);

                // 評価結果を取り出す
                buf.push(pop(RAX));
                self.dec_stack_len();

                // エピローグ
                self.gen_epilogue(buf);
            }

            Stmt::If(StmtIf {
                cond,
                then_branch,
                else_branch: None,
                ..
            }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(cond, buf);

                // 評価結果を取り出す
                buf.push(pop(RAX));
                self.dec_stack_len();

                // 取り出した値が0と等しいかどうか
                buf.push(cmp(RAX, 0));

                // 等しければ一連のコードの終わりにjumpする
                // つまり、以下の処理をスキップする
                let end_label = format!("L_if_end_{}", self.root_gen.new_label_num());
                buf.push(arbitrary(format!("  je {}", end_label)));

                // stmtを評価する
                // `expr` の評価結果が0ならこのコードはスキップされる
                self.gen_stmt(then_branch, buf);

                // ジャンプ先
                buf.push(arbitrary(format!("{}:", end_label)));
            }

            Stmt::If(StmtIf {
                cond,
                then_branch,
                else_branch: Some((_, else_branch)),
                ..
            }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(cond, buf);

                // 評価結果を取り出す
                buf.push(pop(RAX));
                self.dec_stack_len();

                // 評価結果が0と等しいかどうか
                buf.push(cmp(RAX, 0));

                // 等しければ `else_label` にjumpする
                let label_num = self.root_gen.new_label_num();
                let else_label = format!("L_if_else_{}", label_num);
                buf.push(arbitrary(format!("  je {}", else_label)));

                // 評価結果がtrueのときに実行されるstmt
                self.gen_stmt(then_branch, buf);

                // 実行が終わったら `end_label` にjumpする
                // つまりelseのstmtをスキップする
                let end_label = format!("L_if_end_{}", label_num);
                buf.push(arbitrary(format!("  jmp {}", end_label)));

                // else_labelのジャンプ先
                buf.push(arbitrary(format!("{}:", else_label)));

                // 評価結果がfalseのときに実行されるstmt
                self.gen_stmt(else_branch, buf);

                // end_labelのジャンプ先
                buf.push(arbitrary(format!("{}:", end_label)));
            }

            Stmt::While(StmtWhile { cond, block, .. }) => {
                // ループの戻る場所を示す
                let label_num = self.root_gen.new_label_num();
                let begin_label = format!("L_loop_begin_{}", label_num);
                buf.push(arbitrary(format!("{}:", begin_label)));

                // ループ判定の式を評価するコード
                self.gen_expr(cond, buf);

                // ループ判定の結果を取り出す
                buf.push(pop(RAX));
                self.dec_stack_len();

                // 判定の結果が0と等しければend_labelにジャンプ
                buf.push(cmp(RAX, 0));
                let end_label = format!("L_loop_end_{}", label_num);
                buf.push(arbitrary(format!("  je {}", end_label)));

                // stmtを実行するコード
                self.gen_stmt(block, buf);

                // ループの先頭に戻る
                buf.push(arbitrary(format!("  jmp {}", begin_label)));

                // ループを抜け出した場所
                buf.push(arbitrary(format!("{}:", end_label)));
            }

            Stmt::Block(StmtBlock { stmts, .. }) => {
                for stmt in stmts {
                    self.gen_stmt(stmt, buf);
                }
            }
        }
    }

    // スタックトップにexprの結果の値を1つ載せるようなコードを生成する
    pub fn gen_expr<'a>(&mut self, expr: &Expr<'a>, buf: &mut AsmBuf) {
        match expr {
            // スタックトップに即値を載せる
            Expr::Num(n) => {
                buf.push(push(n.num as i64));
                self.inc_stack_len();
            }

            // スタックトップに変数の値を載せる
            Expr::Ident(ExprIdent { ident_offset, .. }) => {
                buf.push(mov(RAX, Addr(RBP) - *ident_offset as i64));
                buf.push(push(RAX));
                self.inc_stack_len();
            }

            // 関数を呼び出す
            Expr::Call(ExprCall {
                ident: func,
                params,
                ..
            }) => {
                if params.len() > 6 {
                    eprintln!("6個より多い引数には対応していません");
                    std::process::exit(1);
                }

                // 引数を評価する
                for param in params.iter() {
                    self.gen_expr(param, buf);
                }

                // 引数をレジスタに載せる
                // スタックには逆順で評価結果が乗っている
                for (i, _param) in params.iter().enumerate().rev() {
                    let reg = match i {
                        0 => RDI,
                        1 => RSI,
                        2 => RDX,
                        3 => RCX,
                        4 => R8,
                        5 => R9,
                        _ => unreachable!(),
                    };

                    buf.push(pop(reg));
                    self.dec_stack_len();
                }

                // RSP を16 byte にalignする
                if self.stack_len % 16 != 0 {
                    buf.push(sub(RSP, 8));
                }

                // 関数の呼び出し
                buf.push(arbitrary(format!("  call _{}", func.name)));
            }

            Expr::Paren(ExprParen { expr, .. }) => self.gen_expr(expr, buf),

            // スタックトップに計算結果を載せる
            Expr::BinOp(ExprBinOp { lhs, op, rhs }) => {
                // スタックトップに1つ値が残る（ようなコードを生成する）
                self.gen_expr(lhs, buf);
                // スタックトップに1つ値が残る（ようなコードを生成する）
                self.gen_expr(rhs, buf);

                // 左ブランチの計算結果をrdiレジスタに記録
                buf.push(pop(RDI));
                self.dec_stack_len();
                // 右ブランチの計算結果をraxレジスタに記録
                buf.push(pop(RAX));
                self.dec_stack_len();

                match op {
                    BinOp::Add(_) => buf.push(add(RAX, RDI)),
                    BinOp::Sub(_) => buf.push(sub(RAX, RDI)),
                    BinOp::Mul(_) => buf.push(imul(RAX, RDI)),
                    BinOp::Div(_) => {
                        buf.push(cqo());
                        buf.push(idiv(RDI));
                    }
                    BinOp::Eq(_) => {
                        // RAXとRDIが等しければZFを立てる
                        buf.push(cmp(RAX, RDI));
                        // ZFが立っていればALに1をセットする
                        buf.push(sete(AL));
                        // ALの値をゼロ拡張してRAXにコピーする
                        buf.push(movzx(RAX, AL));
                    }
                    BinOp::Neq(_) => {
                        // RAXとRDIが等しければZFを立てる
                        buf.push(cmp(RAX, RDI));
                        // ZFが立っていなければALに1をセットする
                        buf.push(setne(AL));
                        // ALの値をゼロ拡張してRAXにコピーする
                        buf.push(movzx(RAX, AL));
                    }
                    BinOp::Lt(_) => {
                        // RAX - RDIの結果をステータスフラグにセットする
                        buf.push(cmp(RAX, RDI));
                        // SF <> OF のときにALに1をセットする
                        buf.push(setl(AL));
                        // ALの値をゼロ拡張してRAXにコピーする
                        buf.push(movzx(RAX, AL));
                    }
                    BinOp::Lte(_) => {
                        // RAXとRDIが等しければZFを立てる
                        buf.push(cmp(RAX, RDI));
                        buf.push(setle(AL));
                        // ALの値をゼロ拡張してRAXにコピーする
                        buf.push(movzx(RAX, AL));
                    }
                }

                buf.push(push(RAX));
                self.inc_stack_len();
            }
        }
    }
}
