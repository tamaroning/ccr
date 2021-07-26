// main.rs

#![allow(dead_code)]

use std::env;
//use std::fmt;

mod tokenize;
mod parse;
mod gen;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        panic!("invalid argument count.");
    }
    
    //println!("Tokenizing...");
    let tokens = tokenize::tokenize(argv[1].clone());
    //println!("Tokenizing...done");

    //println!("parsing...");
    let asts = parse::parse(tokens);
    //println!("Parsing...done");
    
    gen::gen_from_program(asts);
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    //println!("{:?}", parse::parse(String::from("a=1; b=2;")));
    
    println!("=== test finished ===");
}
