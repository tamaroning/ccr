// gen.rs

use std::fs::File;
use std::io::{Write, BufWriter};

use crate::parse::AST;
use crate::parse::NodeKind;

// ASTの配列からアセンブリ全体を生成する
pub fn gen_from_program(vec: Vec<AST>, fname: &str) {

    // 出力ファイルを用意する
    let mut f = BufWriter::new(File::create(fname).unwrap());

    writeln!(f, ".intel_syntax noprefix").unwrap();
    writeln!(f, ".global main").unwrap();
    writeln!(f, "main:").unwrap();
    writeln!(f, "    push rbp").unwrap();
    writeln!(f, "    mov rbp, rsp").unwrap();
    writeln!(f, "    sub rsp, 208").unwrap();

    for elm in vec {
        gen_from_ast(&mut f, elm);
        // 式の評価結果として一つ値が残る
        writeln!(f, "    pop rax").unwrap();
    }

    writeln!(f, "    mov rsp, rbp").unwrap();
    writeln!(f, "    pop rbp").unwrap();
    writeln!(f, "    ret").unwrap();
}

// 左辺値をスタックトップに詰むアセンブリを出力
fn gen_lval(f: &mut BufWriter<File>, ast: AST) {
    match ast {
        AST::Node{ kind: NodeKind::Lvar{offset: ofs, ..}, .. } => {
            writeln!(f, "    mov rax, rbp").unwrap();
            writeln!(f, "    sub rax, {}", ofs).unwrap();
            writeln!(f, "    push rax;").unwrap();
        },
        _ => {
            panic!("代入の左辺値が変数ではありません");
        },
    };
}

// 一つのASTからアセンブリを生成する
pub fn gen_from_ast(f: &mut BufWriter<File>, ast: AST) {

    match ast.clone() {

        AST::Node{ kind: k, lhs: l, rhs: r } => {
            
            match k {
                NodeKind::Num(i) => {
                    writeln!(f, "    push {}", i).unwrap();
                    return;
                },
                NodeKind::Lvar{ .. } => {
                    gen_lval(f, ast);
                    writeln!(f, "    pop rax").unwrap();
                    writeln!(f, "    mov rax, [rax]").unwrap();
                    writeln!(f, "    push rax").unwrap();
                    return;
                }
                NodeKind::Assign => {
                    gen_lval(f, *l);
                    gen_from_ast(f, *r);
                    writeln!(f, "    pop rdi").unwrap();
                    writeln!(f, "    pop rax").unwrap();
                    writeln!(f, "    mov [rax], rdi").unwrap();
                    writeln!(f, "    push rdi").unwrap();
                    return;
                }
                _ => (),
            };
            
            gen_from_ast(f, *l);
            gen_from_ast(f, *r);

            writeln!(f, "    pop rdi").unwrap();
            writeln!(f, "    pop rax").unwrap();

            match k {
                NodeKind::Plus => { writeln!(f, "    add rax, rdi").unwrap(); },
                NodeKind::Minus => { writeln!(f, "    sub rax, rdi").unwrap(); },
                NodeKind::Mul => { writeln!(f, "    imul rax, rdi").unwrap(); },
                NodeKind::Div => {
                    writeln!(f, "    cqo").unwrap();
                    writeln!(f, "    idiv rdi").unwrap();
                },
                NodeKind::Eq => {
                    writeln!(f, "    cmp rax, rdi").unwrap();
                    writeln!(f, "    sete al").unwrap();
                    writeln!(f, "    movzb rax, al").unwrap();
                },
                NodeKind::Ne => {
                    writeln!(f, "    cmp rax, rdi").unwrap();
                    writeln!(f, "    setne al").unwrap();
                    writeln!(f, "    movzb rax, al").unwrap();
                },
                NodeKind::Lt => {
                    writeln!(f, "    cmp rax, rdi").unwrap();
                    writeln!(f, "    setl al").unwrap();
                    writeln!(f, "    movzb rax, al").unwrap();
                },
                NodeKind::Le => {
                    writeln!(f, "    cmp rax, rdi").unwrap();
                    writeln!(f, "    setle al").unwrap();
                    writeln!(f, "    movzb rax, al").unwrap();
                },
                _ => (),  
            };
            writeln!(f, "    push rax").unwrap();

            return;
        },
        _ => ()
    };
}
