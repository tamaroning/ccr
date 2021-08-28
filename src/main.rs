// main.rs

#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
#[allow(unused_imports)]
use std::fmt;

mod tokenize;
mod parse;
mod codegen;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        panic!("invalid argument count.");
    }

    // prepare the source file
    let src_path = Path::new(&argv[1]);
    let src_display = src_path.display();

    // open path as read-only
    let mut src_file = match File::open(&src_path) {
        Err(why) => panic!("couldn't open {}: {}", src_display, why.to_string()),
        Ok(file) => file,
    };

    // load the content of the soure file
    let mut src_string = String::new();
    match src_file.read_to_string(&mut src_string) {
        Err(why) => panic!("couldn't read {}: {}", src_display, why.to_string()),
        Ok(_) => (),//print!("{} contains:\n{}", src_display, src_string),
    }

    // tokenize the source code
    //println!("Tokenizing input...");
    let tokens = tokenize::tokenize(src_string);
    //println!("Done");
    println!("{:?}", tokens);

    // generate AST with Token list
    //println!("Parsing tokens...");
    let asts = parse::parse(tokens);
    //println!("Done");
    println!("{:?}", asts);
    
    // generate the assembly with AST list, then write it to tmp.s
    //println!("Generating assembly...");
    codegen::codegen(asts, "tmp.s");
    //println!("Done");
}

#[test]
fn test_func () {

}
