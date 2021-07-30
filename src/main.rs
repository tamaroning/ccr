// main.rs

#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
//use std::fmt;

mod tokenize;
mod parse;
mod codegen;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        panic!("invalid argument count.");
    }

    // 入力ファイルを準備する
    let src_path = Path::new(&argv[1]);
    let src_display = src_path.display();

    // pathを読み込み専用で開く
    let mut src_file = match File::open(&src_path) {
        Err(why) => panic!("couldn't open {}: {}", src_display, why.to_string()),
        Ok(file) => file,
    };

    // ファイルの中身を読み込む
    let mut src_string = String::new();
    match src_file.read_to_string(&mut src_string) {
        Err(why) => panic!("couldn't read {}: {}", src_display, why.to_string()),
        Ok(_) => print!("{} contains:\n{}", src_display, src_string),
    }

    // ソースコードをトークナイズする
    //println!("Tokenizing input...");
    let tokens = tokenize::tokenize(src_string);
    //println!("Done");
    //println!("{:?}", tokens);

    // トークンの配列からASTを作成
    //println!("Parsing tokens...");
    let asts = parse::parse(tokens);
    //println!("Done");
    //println!("{:?}", asts);
    
    // ASTからアセンブリを生成して,tmp.sに書き込む
    //println!("Generating assembly...");
    codegen::codegen(asts, "tmp.s");
    //println!("Done");
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    
    println!("=== test finished ===");
}
