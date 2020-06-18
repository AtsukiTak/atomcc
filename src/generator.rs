use crate::{
    asm::{instructions::*, Addr, Asm as _, Reg64::*},
    parser::{AssignNode, ExprNode, IfElseNode, IfNode, Node, OpNode},
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

    /// １つのstmtを処理するようなコードを生成する
    pub fn gen(&mut self, node: &Node) {
        match node {
            Node::Expr(expr) => self.gen_expr(expr),

            // ローカル変数にスタックトップの値を代入する
            Node::Assign(AssignNode {
                lhs_ident_offset,
                rhs,
            }) => {
                self.gen_expr(rhs);
                Pop(RAX).print();
                Mov(Addr(RBP) - *lhs_ident_offset as i64, RAX).print();
            }

            Node::Return(expr) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr);

                // 評価結果を取り出す
                Pop(RAX).print();

                // エピローグ
                Mov(RSP, RBP).print();
                Pop(RBP).print();
                println!("  ret");
            }

            Node::If(IfNode { expr, stmt }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr);

                // 評価結果を取り出す
                Pop(RAX).print();

                // 取り出した値が0と等しいかどうか
                println!("  cmp rax, 0");

                // 等しければ一連のコードの終わりにjumpする
                // つまり、以下の処理をスキップする
                let end_label = format!("Lend{}", self.new_label_num());
                println!("  je {}", end_label);

                // stmtを評価する
                // `expr` の評価結果が0ならこのコードはスキップされる
                self.gen(stmt);

                // ジャンプ先
                println!("{}:", end_label);
            }

            Node::IfElse(IfElseNode {
                expr,
                if_stmt,
                else_stmt,
            }) => {
                // 式を評価する（ようなコードを生成する）
                self.gen_expr(expr);

                // 評価結果を取り出す
                Pop(RAX).print();

                // 評価結果が0と等しいかどうか
                println!("  cmp rax, 0");

                // 等しければ `else_label` にjumpする
                let else_label = format!("Lelse{}", self.new_label_num());
                println!("  je {}", else_label);

                // 評価結果がtrueのときに実行されるstmt
                self.gen(if_stmt);

                // 実行が終わったら `end_label` にjumpする
                // つまりelseのstmtをスキップする
                let end_label = format!("Lend{}", self.new_label_num());
                println!("  jmp {}", end_label);

                // else_labelのジャンプ先
                println!("{}:", else_label);

                // 評価結果がfalseのときに実行されるstmt
                self.gen(else_stmt);

                // end_labelのジャンプ先
                println!("{}:", end_label);
            }
        }
    }

    // スタックトップにexprの結果の値を1つ載せるようなコードを生成する
    pub fn gen_expr(&mut self, node: &ExprNode) {
        match node {
            // スタックトップに即値を載せる
            ExprNode::Num(n) => Push(*n as i64).print(),

            // スタックトップに変数の値を載せる
            ExprNode::Ident { offset } => {
                Mov(RAX, Addr(RBP) - *offset as i64).print();
                Push(RAX).print();
            }

            // スタックトップに計算結果を載せる
            ExprNode::Op(OpNode { kind, lhs, rhs }) => {
                self.gen_expr(lhs); // スタックトップに1つ値が残る（ようなコードを生成する）
                self.gen_expr(rhs); // スタックトップに1つ値が残る（ようなコードを生成する）

                Pop(RDI).print(); // 左ブランチの計算結果をrdiレジスタに記録
                Pop(RAX).print(); // 右ブランチの計算結果をraxレジスタに記録

                match kind {
                    Op::Add => println!("  add rax, rdi"),
                    Op::Sub => Sub(RAX, RDI).print(),
                    Op::Mul => println!("  imul rax, rdi"),
                    Op::Div => {
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    Op::Eq => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzx rax, al");
                    }
                    Op::Neq => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzx rax, al");
                    }
                    Op::Lt => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzx rax, al");
                    }
                    Op::Lte => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzx rax, al");
                    }
                    _ => unreachable!(),
                }

                Push(RAX).print();
            }
        }
    }
}
