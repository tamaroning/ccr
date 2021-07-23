// main.rs

#![allow(dead_code)]

use std::env;
//use std::fmt;

mod parse;
mod gen;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        println!("invalid argument count.");
        return;
    }

    println!(".intel_syntax noprefix");
    println!(".section __TEXT,__text");
    println!(".global _main");
    println!("_main:");
    
    gen::gen(parse::parse(argv[1].clone()));
    
    println!("    pop rax");
    println!("    ret");
    
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    println!("=== test finished ===");
}
