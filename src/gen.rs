// gen.rs

use crate::parse::AST;
use crate::parse::NodeKind;

// ASTの配列からアセンブリ全体を生成する
pub fn gen_from_program(vec: Vec<AST>) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("    push rbp");
    println!("    mov rbp, rsp");
    println!("    sub rsp, 208");

    for elm in vec {
        gen_from_ast(elm);
        // 式の評価結果として一つ値が残る
        println!("    pop rax");
    }

    println!("    mov rsp, rbp");
    println!("    pop rbp");
    println!("    ret");
}

// 左辺値をスタックトップに詰むアセンブリを出力
fn gen_lval(ast: AST) {
    match ast {
        AST::Node{ kind: NodeKind::Lvar{offset: ofs, ..}, .. } => {
            println!("    mov rax, rbp");
            println!("    sub rax, {}", ofs);
            println!("    push rax;")
        },
        _ => {
            panic!("代入の左辺値が変数ではありません");
        },
    };
}

// 一つのASTからアセンブリを生成する
pub fn gen_from_ast(ast: AST) {

    match ast.clone() {

        AST::Node{ kind: k, lhs: l, rhs: r } => {
            
            match k {
                NodeKind::Num(i) => {
                    println!("    push {}", i);
                    return;
                },
                NodeKind::Lvar{ .. } => {
                    gen_lval(ast);
                    println!("    pop rax");
                    println!("    mov rax, [rax]");
                    println!("    push rax");
                    return;
                }
                NodeKind::Assign => {
                    gen_lval(*l);
                    gen_from_ast(*r);
                    println!("    pop rdi");
                    println!("    pop rax");
                    println!("    mov [rax], rdi");
                    println!("    push rdi");
                    return;
                }
                _ => (),
            };
            
            gen_from_ast(*l);
            gen_from_ast(*r);

            println!("    pop rdi");
            println!("    pop rax");

            match k {
                NodeKind::Plus => { println!("    add rax, rdi"); },
                NodeKind::Minus => { println!("    sub rax, rdi"); },
                NodeKind::Mul => { println!("    imul rax, rdi"); },
                NodeKind::Div => {
                    println!("    cqo");
                    println!("    idiv rdi");
                },
                NodeKind::Eq => {
                    println!("    cmp rax, rdi");
                    println!("    sete al");
                    println!("    movzb rax, al");
                },
                NodeKind::Ne => {
                    println!("    cmp rax, rdi");
                    println!("    setne al");
                    println!("    movzb rax, al");
                },
                NodeKind::Lt => {
                    println!("    cmp rax, rdi");
                    println!("    setl al");
                    println!("    movzb rax, al");
                },
                NodeKind::Le => {
                    println!("    cmp rax, rdi");
                    println!("    setle al");
                    println!("    movzb rax, al");
                },
                _ => (),  
            };
            println!("    push rax");

            return;
        },
        _ => ()
    };
}
