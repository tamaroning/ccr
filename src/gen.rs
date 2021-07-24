// gen.rs

use crate::parse::AST;
use crate::parse::NodeKind;

pub fn gen_assembly(ast: AST) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen_from_ast(ast);
}

pub fn gen_from_ast(ast: AST) {

    match ast {
        AST::Node{ kind: NodeKind::Num(i), .. } => {
            println!("    push {}", i);
            return;
        },
        AST::Node{ kind: k, lhs: l, rhs: r } => {
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
