use crate::{
    asm::{arbitrary, instructions::*, Addr, AsmBuf, Reg64::*},
    parser::{AssignNode, ExprNode, IfElseNode, IfNode, Node, OpNode, WhileNode},
    token::Op,
};

pub struct Generator {
    next_label_num: usize,
}

impl Generator {
    pub fn new() -> Self {
        Generator { next_label_num: 0 }
    }

    fn new_label_num(&mut self) -> usize {
        let n = self.next_label_num;
        self.next_label_num += 1;
        n
    }

    pub fn gen(&mut self, nodes: &[Node], asm_buf: &mut AsmBuf) {
        self.gen_prelude(asm_buf);
        self.gen_prologue(26, asm_buf);
        for node in nodes {
            self.gen_stmt(node, asm_buf);
        }
        // 最後にスタックに残っていた値をRAXレジスタに保存
        asm_buf.push(pop(RAX));
        self.gen_epilogue(asm_buf);
    }

    pub fn gen_prelude(&self, asm_buf: &mut AsmBuf) {
        asm_buf.push(arbitrary(".intel_syntax noprefix"));
        asm_buf.push(arbitrary(".global _main"));
        asm_buf.push(arbitrary("_main:"));
    }

    pub fn gen_prologue(&self, stack_bytes: i64, asm_buf: &mut AsmBuf) {
        // ベースポインタの値を避難
        asm_buf.push(push(RBP));
        // ベースポインタを、スタックポインタまで移動
        asm_buf.push(mov(RBP, RSP));
        // stack領域の確保 (スタックポインタの移動)
        asm_buf.push(sub(RSP, 8 * stack_bytes));
    }

    pub fn gen_epilogue(&self, asm_buf: &mut AsmBuf) {
        // スタックポインタをベースポインタまで移動
        asm_buf.push(mov(RSP, RBP));
        // prologueで避難させておいたベースポインタの値を戻す
        asm_buf.push(pop(RBP));
        asm_buf.push(arbitrary("  ret"));
    }

    /// １つのstmtを処理するようなコードを生成する
    pub fn gen_stmt(&mut self, node: &Node, asm_buf: &mut AsmBuf) {
        match node {
            Node::Expr(expr) => self.gen_expr(expr, asm_buf),

            // ローカル変数にスタックトップの値を代入する
            Node::Assign(AssignNode {
                lhs_ident_offset,
                rhs,
            }) => {
                self.gen_expr(rhs, asm_buf);
                asm_buf.push(pop(RAX));
                asm_buf.push(mov(Addr(RBP) - *lhs_ident_offset as i64, RAX));
            }

            Node::Return(expr) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr, asm_buf);

                // 評価結果を取り出す
                asm_buf.push(pop(RAX));

                // エピローグ
                asm_buf.push(mov(RSP, RBP));
                asm_buf.push(pop(RBP));
                asm_buf.push(arbitrary("  ret"));
            }

            Node::If(IfNode { expr, stmt }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr, asm_buf);

                // 評価結果を取り出す
                asm_buf.push(pop(RAX));

                // 取り出した値が0と等しいかどうか
                asm_buf.push(arbitrary("  cmp rax, 0"));

                // 等しければ一連のコードの終わりにjumpする
                // つまり、以下の処理をスキップする
                let end_label = format!("if_end_{}", self.new_label_num());
                asm_buf.push(arbitrary(format!("  je {}", end_label)));

                // stmtを評価する
                // `expr` の評価結果が0ならこのコードはスキップされる
                self.gen_stmt(stmt, asm_buf);

                // ジャンプ先
                asm_buf.push(arbitrary(format!("{}:", end_label)));
            }

            Node::IfElse(IfElseNode {
                expr,
                if_stmt,
                else_stmt,
            }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr, asm_buf);

                // 評価結果を取り出す
                asm_buf.push(pop(RAX));

                // 評価結果が0と等しいかどうか
                asm_buf.push(arbitrary("  cmp rax, 0"));

                // 等しければ `else_label` にjumpする
                let label_num = self.new_label_num();
                let else_label = format!("if_else_{}", label_num);
                asm_buf.push(arbitrary(format!("  je {}", else_label)));

                // 評価結果がtrueのときに実行されるstmt
                self.gen_stmt(if_stmt, asm_buf);

                // 実行が終わったら `end_label` にjumpする
                // つまりelseのstmtをスキップする
                let end_label = format!("if_end_{}", label_num);
                asm_buf.push(arbitrary(format!("  jmp {}", end_label)));

                // else_labelのジャンプ先
                asm_buf.push(arbitrary(format!("{}:", else_label)));

                // 評価結果がfalseのときに実行されるstmt
                self.gen_stmt(else_stmt, asm_buf);

                // end_labelのジャンプ先
                asm_buf.push(arbitrary(format!("{}:", end_label)));
            }

            Node::While(WhileNode { cond, stmt }) => {
                // ループの戻る場所を示す
                let label_num = self.new_label_num();
                let begin_label = format!("loop_begin_{}", label_num);
                asm_buf.push(arbitrary(format!("{}:", begin_label)));

                // ループ判定の式を評価するコード
                self.gen_expr(cond, asm_buf);

                // ループ判定の結果を取り出す
                asm_buf.push(pop(RAX));

                // 判定の結果が0と等しければend_labelにジャンプ
                asm_buf.push(arbitrary("  cmp rax, 0"));
                let end_label = format!("loop_end_{}", label_num);
                asm_buf.push(arbitrary(format!("  je {}", end_label)));

                // stmtを実行するコード
                self.gen_stmt(stmt, asm_buf);

                // ループの先頭に戻る
                asm_buf.push(arbitrary(format!("  jmp {}", begin_label)));

                // ループを抜け出した場所
                asm_buf.push(arbitrary(format!("{}:", end_label)));
            }
        }
    }

    // スタックトップにexprの結果の値を1つ載せるようなコードを生成する
    pub fn gen_expr(&mut self, node: &ExprNode, asm_buf: &mut AsmBuf) {
        match node {
            // スタックトップに即値を載せる
            ExprNode::Num(n) => asm_buf.push(push(*n as i64)),

            // スタックトップに変数の値を載せる
            ExprNode::Ident { offset } => {
                asm_buf.push(mov(RAX, Addr(RBP) - *offset as i64));
                asm_buf.push(push(RAX));
            }

            // スタックトップに計算結果を載せる
            ExprNode::Op(OpNode { kind, lhs, rhs }) => {
                self.gen_expr(lhs, asm_buf); // スタックトップに1つ値が残る（ようなコードを生成する）
                self.gen_expr(rhs, asm_buf); // スタックトップに1つ値が残る（ようなコードを生成する）

                asm_buf.push(pop(RDI)); // 左ブランチの計算結果をrdiレジスタに記録
                asm_buf.push(pop(RAX)); // 右ブランチの計算結果をraxレジスタに記録

                match kind {
                    Op::Add => asm_buf.push(arbitrary("  add rax, rdi")),
                    Op::Sub => asm_buf.push(sub(RAX, RDI)),
                    Op::Mul => asm_buf.push(arbitrary("  imul rax, rdi")),
                    Op::Div => {
                        asm_buf.push(arbitrary("  cqo"));
                        asm_buf.push(arbitrary("  idiv rdi"));
                    }
                    Op::Eq => {
                        asm_buf.push(arbitrary("  cmp rax, rdi"));
                        asm_buf.push(arbitrary("  sete al"));
                        asm_buf.push(arbitrary("  movzx rax, al"));
                    }
                    Op::Neq => {
                        asm_buf.push(arbitrary("  cmp rax, rdi"));
                        asm_buf.push(arbitrary("  setne al"));
                        asm_buf.push(arbitrary("  movzx rax, al"));
                    }
                    Op::Lt => {
                        asm_buf.push(arbitrary("  cmp rax, rdi"));
                        asm_buf.push(arbitrary("  setl al"));
                        asm_buf.push(arbitrary("  movzx rax, al"));
                    }
                    Op::Lte => {
                        asm_buf.push(arbitrary("  cmp rax, rdi"));
                        asm_buf.push(arbitrary("  setle al"));
                        asm_buf.push(arbitrary("  movzx rax, al"));
                    }
                    _ => unreachable!(),
                }

                asm_buf.push(push(RAX));
            }
        }
    }
}
