// gen.rs

use crate::parse::AST;
use crate::parse::NodeKind;

pub fn gen(ast: AST) {
    match ast {
        AST::Node{ kind: NodeKind::Num(i), .. } => {
            println!("    push {}", i);
            return;
        },
        AST::Node{ kind: k, lhs: l, rhs: r } => {
            gen(*l);
            gen(*r);

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
                _ => (),  
            };
            println!("    push rax");

            return;
        },
        _ => ()
    };
}
