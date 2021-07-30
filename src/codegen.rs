// gen.rs

use std::fs::File;
use std::io::{Write, BufWriter};

use crate::parse::AST;
use crate::parse::NodeKind;

const ARGREG: [&'static str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

#[derive(Debug)]
struct CodeGenerator {
    ast_list: Vec<AST>,
    label_cnt: usize,
    f: BufWriter<File>, 
}

// ASTの配列からアセンブリ全体を生成する
pub fn codegen(vec: Vec<AST>, fname: &str) {

    let mut gen = CodeGenerator{ ast_list: vec, label_cnt: 0, f: BufWriter::new(File::create(fname).unwrap()) };

    gen.output(".intel_syntax noprefix");
    gen.output(".global main");
    gen.output("main:");
    gen.output("    push rbp");
    gen.output("    mov rbp, rsp");

    // スタックフレームを用意する
    gen.output("    sub rsp, 208");
    // vecの各要素(stmt)からアセンブリを生成する。
    for elm in gen.ast_list.clone() {
        gen.output("# stmt begin");
        gen.gen_stmt(elm);
        // 式の評価結果として一つ値が残る
        gen.output("    pop rax");
        gen.output("# stmt end");
    }

}

fn is_nil(ast: AST) -> bool {
    match ast {
        AST::Nil => true,
        _ => false,
    }
}

// 変数かどうか
fn is_var(ast: AST) -> bool {
    match ast {
        AST::Node{ kind: NodeKind::Var{ .. }, .. } => true,
        _ => false,
    }
}

impl CodeGenerator {
    // ファイルへの書き出し
    fn output(&mut self, s: &str) {
        writeln!(self.f, "{}",s).unwrap();
    }

    // exprからアセンブリを出力する
    pub fn gen_expr(&mut self, ast: AST) {
        match ast.clone() {
            AST::Node{ kind: k, lhs: l, rhs: r } => {
                match k {
                    // 代入式
                    // 左辺はローカル変数, 右辺はexpr
                    NodeKind::Assign => {
                        self.output("# assign begin");
                        // 左辺のアドレスをスタックトップに詰む
                        match *l.clone() {
                            AST::Node{ kind: NodeKind::Var{ .. }, .. } => self.gen_addr(*l),
                            AST::Node{ kind: NodeKind::Deref, lhs: deref_l, .. } => self.gen_expr(*deref_l),
                            _ => panic!("代入式の左辺が間違っています"),
                        }
                        // 右辺値の結果をスタックトップに詰む
                        self.gen_expr(*r);
                        // 左辺に右辺を代入する
                        self.output("    pop rdi");
                        self.output("    pop rax");
                        self.output("    mov [rax], rdi");
                        self.output("    push rdi");
                        self.output("# assign end");
                        return;
                    },
                    // 関数呼び出し
                    NodeKind::FuncCall{ name: func_name, argv: args } => {
                        // 関数呼び出し直前にrspを16の倍数にアラインするアセンブリをかく
                        // 引数をレジスタに格納する
                        for i in 0..=5 {
                            if args.len() > 5-i {
                                self.gen_expr(args[5-i].clone());
                                self.output("    pop rax");
                                self.output(&format!("    mov {}, rax", ARGREG[5-i]));
                            }
                        }

                        if args.len() > 6 { panic!("the number of arguments must be < 7")}
                        
                        self.output(&format!("    call {}", func_name));
                        self.output("    push rax");
                        return;
                    },
                    // 整数
                    // 左辺と右辺はNil
                    NodeKind::Num(i) => {
                        self.output(&format!("    push {};", i));
                        return;
                    },
                    // ローカル変数
                    // 左辺と右辺はNil
                    NodeKind::Var{ .. } => {
                        self.output("# Var begin");
                        // 左辺値をスタックトップに積む
                        self.gen_addr(ast);
                        // rax <- 左辺値
                        self.output("    pop rax");
                        self.output("    mov rax, [rax]");
                        self.output("    push rax");
                        self.output("# Var end");
                        return;
                    }
                    NodeKind::Deref => {
                        self.gen_expr(*l);
                        self.output("    pop rax");
                        self.output("    mov rax, [rax]");
                        self.output("    push rax");
                        return;
                    },
                    NodeKind::Addr => {
                        // *&xや*(&x+8)に対応する
                        if is_var(*l.clone()) {
                            self.gen_addr(*l);
                        } else {
                            self.gen_expr(*l);
                        }
                        return;
                    },
                    _ => (),
                };
                
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi"); // 右辺値
                self.output("    pop rax"); // 左辺値

                // rax <- 右辺値と左辺値の演算結果
                match k {
                    // 四則演算
                    NodeKind::Plus => { self.output("    add rax, rdi"); },
                    NodeKind::Minus => { self.output("    sub rax, rdi"); },
                    NodeKind::Mul => { self.output("    imul rax, rdi"); },
                    NodeKind::Div => {
                        self.output("    cqo");
                        self.output("    idiv rdi");
                    },
                    // 比較演算子
                    NodeKind::Eq => {
                        self.output("    cmp rax, rdi");
                        self.output("    sete al");
                        self.output("    movzb rax, al");
                    },
                    NodeKind::Ne => {
                        self.output("    cmp rax, rdi");
                        self.output("    setne al");
                        self.output("    movzb rax, al");
                    },
                    NodeKind::Lt => {
                        self.output("    cmp rax, rdi");
                        self.output("    setl al");
                        self.output("    movzb rax, al");
                    },
                    NodeKind::Le => {
                        self.output("    cmp rax, rdi");
                        self.output("    setle al");
                        self.output("    movzb rax, al");
                    },
                    _ => (),  
                };

                // rax をスタックトップに積む
                self.output("    push rax");

                return;
            },
            _ => (),//panic!("expr must not be nil"), // Nil
        };
    }

    fn gen_stmt(&mut self, ast: AST) {

        if is_nil(ast.clone()) { panic!("incorrect statement"); }
        

        match ast.kind() {
            NodeKind::Return => {
                self.output("# return begin");
                self.output("# value begin");
                self.gen_expr(*ast.lhs()); // returnノードのrhsはNilを指す
                self.output("# value end");
                self.output("    pop rax");
                self.output("    mov rsp, rbp");
                self.output("    pop rbp");
                self.output("    ret"); // 簡単のためにretが複数出力されることがある
                self.output("# return end");
                return;
            },
            NodeKind::If{ cond: c, then: t, els: e } => {
                // elseなし
                match *e {
                    // else あり
                    AST::Node{ .. } => {
                        let label_else = format!(".Lelse{}", self.label_cnt);
                        self.label_cnt += 1;
                        let label_end = format!(".Lelse{}", self.label_cnt);
                        self.label_cnt += 1;

                        self.gen_expr(*c);
                        self.output("    pop rax");
                        self.output("    cmp rax, 0");
                        self.output(&format!("    je {}", label_else));
                        self.gen_stmt(*t);
                        self.output(&format!("    jmp {}", label_end));
                        self.output(&format!("{}:", label_else));
                        self.gen_stmt(*e);
                        self.output(&format!("{}:", label_end));
                        return;
                    },
                    // else なし
                    _ => {
                        let label = format!(".Lend{}", self.label_cnt);
                        self.label_cnt += 1;

                        self.gen_expr(*c);
                        self.output("    pop rax");
                        self.output("    cmp rax, 0");
                        self.output(&format!("    je {}", label));
                        self.gen_stmt(*t);
                        self.output(&format!("{}:", label));
                        return;
                    }
                };
            },
            NodeKind::For{ a: expr_a, b: expr_b, c: expr_c, proc: p } => {
                let label_begin = format!(".Lbegin{}", self.label_cnt);
                self.label_cnt += 1;
                let label_end = format!(".Lend{}", self.label_cnt);
                self.label_cnt += 1;
                // Nilを許容
                if !is_nil(*expr_a.clone()) { self.gen_expr(*expr_a); }
                self.output(&format!("{}:", label_begin));
                if !is_nil(*expr_b.clone()) {
                    self.gen_expr(*expr_b);
                    self.output("    pop rax");
                    self.output("    cmp rax, 0");
                    self.output(&format!("    je {}", label_end));
                }
                self.gen_stmt(*p);
                if !is_nil(*expr_c.clone()) { self.gen_expr(*expr_c); }
                self.output(&format!("    jmp {}", label_begin));
                self.output(&format!("{}:", label_end));
                return;
            },
            NodeKind::Block(vec) => {
                for ast in *vec {
                    self.gen_stmt(ast);
                }
                return;
            },
            // 式からなる文
            NodeKind::ExprStmt(expr) => {
                self.gen_expr(*expr);
            },
            _ => panic!("incorrect statement"),
        };

    }

    // Lvarノードのアドレスをプッシュする
    fn gen_addr(&mut self, ast: AST) {
        match ast {
            AST::Node{ kind: NodeKind::Var{ offset: ofs, ..}, .. } => {
                // ローカル変数のアドレスをスタックに積む
                self.output("    mov rax, rbp");
                self.output(&format!("    sub rax, {}", ofs));
                self.output("    push rax");
            },
            _ => {
                panic!("変数ではないノードにアドレスはありません");
            },
        };
    }

}