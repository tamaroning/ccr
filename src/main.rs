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
    
    gen::gen_from_program(parse::parse(argv[1].clone()));
    
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    println!("{:?}", parse::parse(String::from("a=1; b=2;")));
    
    println!("=== test finished ===");
}
