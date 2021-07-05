#![allow(dead_code)]

use std::env;


fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        println!("invalid argument count.");
        return;
    }

    let mut parser = Parser { pos: 0, input: argv[1].clone(),};

    println!(".intel_syntax noprefix");
    println!(".section __TEXT,__text");
    println!(".global _main");
    println!("_main:");
    println!("    mov rax, {}", parser.parse_uint());
    
    while !parser.eof() {
        if parser.next_char() == '+' {
            parser.consume_char();
            println!("    add rax, {}", parser.parse_uint());
            continue;
        }
        else if parser.next_char() == '-' {
            parser.consume_char();
            println!("    sub rax, {}", parser.parse_uint());
            continue;
        }
        println!("unexpected char: {}", parser.next_char());
        return;
    }

    println!("    ret");

}

#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }
            return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_uint(&mut self) -> u32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });
        s.parse::<u32>().unwrap()
    }
}

