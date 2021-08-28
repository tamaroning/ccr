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
    // set the currrent directory
    let fpath = std::env::current_exe().unwrap().parent().unwrap().join(fname);
    let mut gen = CodeGenerator{ ast_list: vec, label_cnt: 0, f: BufWriter::new(File::create(&fpath).unwrap()) };

    gen.output(".intel_syntax noprefix");
    gen.output(".global main");

    // vecの各要素(stmt)からアセンブリを生成する。
    for elm in gen.ast_list.clone() {
        gen.gen_no_ret(elm);
    }

}

fn is_nil(ast: AST) -> bool {
    match ast {
        AST::Nil => true,
        _ => false,
    }
}

fn is_var(ast: AST) -> bool {
    match ast {
        AST::Node{ kind: NodeKind::Var{ .. } } => true,
        _ => false,
    }
}

impl CodeGenerator {
    // write str to the file
    fn output(&mut self, s: &str) {
        writeln!(self.f, "{}",s).unwrap();
    }

    // push address of variables
    fn gen_addr(&mut self, ast: AST) {
        match ast {
            AST::Node{ kind: NodeKind::Var{ offset: ofs, ..} } => {
                self.output("    mov rax, rbp");
                self.output(&format!("    sub rax, {}", ofs));
                self.output("    push rax");
            },
            _ => {
                panic!("non variable nodes doesn't have addresses");
            },
        };
    }

    // exprからアセンブリを出力する　Nilは受け付けない
    pub fn gen_expr(&mut self, ast: AST) {
        if is_nil(ast.clone()) { panic!("Nil is not expr"); }

        let k = ast.kind().clone();

        match k {
            // assignment
            // lhs is variables or deref*, rhs is expression
            NodeKind::Assign(l,r) => {
                // push the address of lhs
                match *l.clone() {
                    AST::Node{ kind: NodeKind::Var{ .. } } => self.gen_addr(*l),
                    AST::Node{ kind: NodeKind::Deref(ast) } => self.gen_expr(*ast),
                    _ => panic!("incorrect assignment: lhs is wrong expression"),
                }
                self.gen_expr(*r); 
                self.output("    pop rdi"); // rhs 
                self.output("    pop rax"); // lhs
                self.output("    mov [rax], rdi");
                self.output("    push rdi");
            },
            // function call
            NodeKind::FuncCall{ name: func_name, argv: args } => {
                // Todo 関数呼び出し直前にrspを16の倍数にアラインするアセンブリをかく
                // store argument in the registers
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
            },
            // integers
            NodeKind::Num(i) => {
                self.output(&format!("    push {}", i));
            },
            // variables
            NodeKind::Var{ .. } => {
                self.gen_addr(ast);
                self.output("    pop rax");
                self.output("    mov rax, [rax]");
                self.output("    push rax");
            }
            NodeKind::Deref(ast) => {
                self.gen_expr(*ast);
                self.output("    pop rax");
                self.output("    mov rax, [rax]");
                self.output("    push rax");
            },
            NodeKind::Addr(ast) => {
                if is_var(*ast.clone()) {
                    self.gen_addr(*ast);
                } else {
                    self.gen_expr(*ast);
                }
            },
            // --- arithmetic operators ---
            NodeKind::Plus(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    add rax, rdi");
                self.output("    push rax");
            },
            NodeKind::Minus(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    sub rax, rdi");
                self.output("    push rax");
            },
            NodeKind::Mul(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    imul rax, rdi"); 
                self.output("    push rax");
            },
            NodeKind::Div(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    cqo");
                self.output("    idiv rdi");
                self.output("    push rax");
            },
            // comparison operators
            NodeKind::Eq(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax"); 
                self.output("    cmp rax, rdi");
                self.output("    sete al");
                self.output("    movzb rax, al");
                self.output("    push rax");
            },
            NodeKind::Ne(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    cmp rax, rdi");
                self.output("    setne al");
                self.output("    movzb rax, al");
                self.output("    push rax");
            },
            NodeKind::Lt(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    cmp rax, rdi");
                self.output("    setl al");
                self.output("    movzb rax, al");
                self.output("    push rax");
            },
            NodeKind::Le(l, r) => {
                self.gen_expr(*l);
                self.gen_expr(*r);
                self.output("    pop rdi");
                self.output("    pop rax");
                self.output("    cmp rax, rdi");
                self.output("    setle al");
                self.output("    movzb rax, al");
                self.output("    push rax");
            },
            _ => (),  
        };
        return;
    }

    // スタックトップに結果が積まれないもの (式以外)
    fn gen_no_ret(&mut self, ast: AST) {

        if is_nil(ast.clone()) { panic!("incorrect statement"); }
        match ast.kind() {
            NodeKind::FuncDecl{ name: func_name, args: func_args, ret_type: _ret_ty, frame_size: func_frame_size, stmts: func_stmts, } => {
                self.output(&format!("{}:", func_name));
                self.output("    push rbp");
                self.output("    mov rbp, rsp");
                // prepare the stack frame
                self.output(&format!("    sub rsp, {}", func_frame_size));

                for i in 0..6 {
                    if func_args.len() >= i+1 { self.output(&format!("    mov [rbp - {}], {}", func_args[i].0, ARGREG[i])); }
                }
                for elm in *func_stmts.clone() {
                    self.gen_no_ret(elm);
                    // one value remains on the stack top as the result of evaluating expression
                }
                
                // ToDo ret_typeでどうこうする

                return;
            },
            NodeKind::Return(ast) => {
                if !is_nil(*ast.clone()) {
                    self.gen_expr(*ast);
                    self.output("    pop rax");
                }
                self.output("    mov rsp, rbp");
                self.output("    pop rbp");
                self.output("    ret");
                return;
            },
            NodeKind::If{ cond: c, then: t, els: e } => {
                // no else 
                if is_nil(*e.clone()){
                    let label = format!(".Lend{}", self.label_cnt);
                    self.label_cnt += 1;

                    self.gen_expr(*c);
                    self.output("    pop rax");
                    self.output("    cmp rax, 0");
                    self.output(&format!("    je {}", label));
                    self.gen_no_ret(*t);
                    self.output(&format!("{}:", label));
                    return;
                }
                // if-else
                else {
                    let label_else = format!(".Lelse{}", self.label_cnt);
                    self.label_cnt += 1;
                    let label_end = format!(".Lelse{}", self.label_cnt);
                    self.label_cnt += 1;

                    self.gen_expr(*c);
                    self.output("    pop rax");
                    self.output("    cmp rax, 0");
                    self.output(&format!("    je {}", label_else));
                    self.gen_no_ret(*t);
                    self.output(&format!("    jmp {}", label_end));
                    self.output(&format!("{}:", label_else));
                    self.gen_no_ret(*e);
                    self.output(&format!("{}:", label_end));
                    return;
                }
            },
            NodeKind::For{ a: expr_a, b: expr_b, c: expr_c, proc: p } => {
                let label_begin = format!(".Lbegin{}", self.label_cnt);
                self.label_cnt += 1;
                let label_end = format!(".Lend{}", self.label_cnt);
                self.label_cnt += 1;
                // Nilを許容
                if !is_nil(*expr_a.clone()) { self.gen_no_ret(*expr_a); }
                self.output(&format!("{}:", label_begin));
                if !is_nil(*expr_b.clone()) {
                    self.gen_expr(*expr_b);
                    self.output("    pop rax");
                    self.output("    cmp rax, 0");
                    self.output(&format!("    je {}", label_end));
                }
                self.gen_no_ret(*p);
                if !is_nil(*expr_c.clone()) { self.gen_expr(*expr_c); }
                self.output(&format!("    jmp {}", label_begin));
                self.output(&format!("{}:", label_end));
                return;
            },
            NodeKind::Block(vec) => {
                for ast in *vec {
                    self.gen_no_ret(ast);
                }
                return;
            },
            // expression statement
            NodeKind::ExprStmt(expr) => {
                self.gen_expr(*expr);
            },
            _ => panic!("incorrect statement"),
        };
    }

}

