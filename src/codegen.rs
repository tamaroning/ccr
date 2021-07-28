// gen.rs

use std::fs::File;
use std::io::{Write, BufWriter};

use crate::parse::AST;
use crate::parse::NodeKind;

// ASTの配列からアセンブリ全体を生成する
pub fn codegen(vec: Vec<AST>, fname: &str) {

    // 出力ファイルを用意する
    let mut f = BufWriter::new(File::create(fname).unwrap());

    writeln!(f, ".intel_syntax noprefix").unwrap();
    writeln!(f, ".global main").unwrap();
    writeln!(f, "main:").unwrap();
    writeln!(f, "    push rbp").unwrap();
    writeln!(f, "    mov rbp, rsp").unwrap();

    // スタックフレームを用意する
    writeln!(f, "    sub rsp, 208").unwrap();

    // vecの各要素(stmt)からアセンブリを生成する。
    for elm in vec {
        gen_expr(&mut f, elm);
        // 式の評価結果として一つ値が残る
        writeln!(f, "    pop rax").unwrap();
    }

    // スタックフレームを戻す
    writeln!(f, "    mov rsp, rbp").unwrap();
    writeln!(f, "    pop rbp").unwrap();
    writeln!(f, "    ret").unwrap();
}

// 代入の左辺値(変数)のアドレスをスタックトップに詰むアセンブリを出力
fn gen_lval(f: &mut BufWriter<File>, ast: AST) {
    match ast {
        AST::Node{ kind: NodeKind::Lvar{offset: ofs, ..}, .. } => {
            // ローカル変数のアドレスをスタックに積む
            // rax <- rbp - ofs
            // push rax
            writeln!(f, "    mov rax, rbp").unwrap();
            writeln!(f, "    sub rax, {}", ofs).unwrap();
            writeln!(f, "    push rax;").unwrap();
        },
        _ => {
            panic!("代入の左辺値が変数ではありません");
        },
    };
}

// exprからアセンブリを出力する
pub fn gen_expr(f: &mut BufWriter<File>, ast: AST) {

    match ast.clone() {

        AST::Node{ kind: k, lhs: l, rhs: r } => {
            match k {
                // 整数
                // 左辺と右辺はNil
                NodeKind::Num(i) => {
                    writeln!(f, "    push {}", i).unwrap();
                    return;
                },
                // ローカル変数
                // 左辺と右辺はNil
                NodeKind::Lvar{ .. } => {
                    // 左辺値をスタックトップに積む
                    gen_lval(f, ast);
                    // rax <- 左辺値
                    writeln!(f, "    pop rax").unwrap();
                    writeln!(f, "    mov rax, [rax]").unwrap();
                    writeln!(f, "    push rax").unwrap();
                    return;
                }
                // 代入式
                // 左辺はローカル変数, 右辺はexpr
                NodeKind::Assign => {
                    // 左辺のアドレスをスタックトップに詰む
                    gen_lval(f, *l);
                    // 右辺値の結果をスタックトップに詰む
                    gen_expr(f, *r);
                    // 左辺に右辺を代入する
                    writeln!(f, "    pop rdi").unwrap();
                    writeln!(f, "    pop rax").unwrap();
                    writeln!(f, "    mov [rax], rdi").unwrap();
                    writeln!(f, "    push rdi").unwrap();
                    return;
                }
                NodeKind::Return => {
                    gen_expr(f, *l); // returnノードのrhsはNilを指す
                    writeln!(f, "       pop rax").unwrap();
                    writeln!(f, "       mov rsp, rbp").unwrap();
                    writeln!(f, "       pop rbp").unwrap();
                    writeln!(f, "       ret").unwrap(); // 簡単のためにretが複数出力されることがある
                    return;
                },
                _ => (),
            };
            
            // rdi <- 右辺値
            // rax <- 左辺値
            gen_expr(f, *l);
            gen_expr(f, *r);
            writeln!(f, "    pop rdi").unwrap();
            writeln!(f, "    pop rax").unwrap();

            // rax <- 右辺値と左辺値の演算結果
            match k {
                // 四則演算
                NodeKind::Plus => { writeln!(f, "    add rax, rdi").unwrap(); },
                NodeKind::Minus => { writeln!(f, "    sub rax, rdi").unwrap(); },
                NodeKind::Mul => { writeln!(f, "    imul rax, rdi").unwrap(); },
                NodeKind::Div => {
                    writeln!(f, "    cqo").unwrap();
                    writeln!(f, "    idiv rdi").unwrap();
                },
                // 比較演算子
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

            // rax をスタックトップに積む
            writeln!(f, "    push rax").unwrap();

            return;
        },
        _ => ()
    };
}
