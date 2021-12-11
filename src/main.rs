#![allow(dead_code)]

use std::env;
#[allow(unused_imports)]
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod codegen;
mod parse;
mod tokenize;

fn main() {
    let mut is_quiet = false;
    let mut is_debug = false;

    let argv: Vec<String> = env::args().collect();

    // prepare the source file
    let mut src_path = Path::new("");
    let src_display = src_path.display();

    // process the arguments
    for arg in &argv {
        if arg == "-q" {
            is_quiet = true;
        } else if arg == "-d" {
            is_debug = true;
        } else {
            src_path = Path::new(arg);
        }
    }

    if src_path.to_str() == Some("") {
        panic!("No input file");
    }

    // open path as read-only
    let mut src_file = match File::open(&src_path) {
        Err(why) => panic!("couldn't open {}: {}", src_display, why.to_string()),
        Ok(file) => file,
    };

    // load the content of the soure file
    let mut src_string = String::new();
    match src_file.read_to_string(&mut src_string) {
        Err(why) => panic!("couldn't read {}: {}", src_display, why.to_string()),
        Ok(_) => (), //print!("{} contains:\n{}", src_display, src_string),
    }

    // tokenize the source code
    if !is_quiet && !is_debug {
        println!("Tokenizing input...");
    }
    let tokens = tokenize::tokenize(src_string);
    if !is_quiet && !is_debug {
        println!("Done");
    }
    if is_debug {
        println!("{:?}", tokens);
    }

    // generate AST with Token list
    if !is_quiet && !is_debug {
        println!("Parsing tokens...");
    }
    let asts = parse::parse(tokens);
    if !is_quiet && !is_debug {
        println!("Done");
    }
    if is_debug {
        println!("{:?}", asts);
    }

    // generate the assembly with AST list, then write it to tmp.s
    if !is_quiet && !is_debug {
        println!("Generating assembly...");
    }
    codegen::codegen(asts, "tmp.s");
    if !is_quiet && !is_debug {
        println!("Done");
    }
}
