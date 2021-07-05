#[allow(dead_code)]

use std::io;
use std::env;

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
    println!("    mov rax, {}", argv[1].parse::<i32>().unwrap());
    println!("    ret");

}
