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
    
    gen::gen_assembly(parse::parse(argv[1].clone()));
    
    println!("    pop rax");
    println!("    ret");
    
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    println!("{:?}", parse::parse(String::from("1 < 2 ==3 >4 ")));
    

    println!("=== test finished ===");
}
