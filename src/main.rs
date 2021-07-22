#![allow(dead_code)]

use std::env;
use std::fmt;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        println!("invalid argument count.");
        return;
    }

    /*
    let mut tokenizer = Parser { pos: 0, input: argv[1].clone(),};

    println!(".intel_syntax noprefix");
    println!(".section __TEXT,__text");
    println!(".global _main");
    println!("_main:");
    println!("    mov rax, {}", tokenizer.consume_number());
    
    while !tokenizer.is_eof() {
        if tokenizer.next_char() == '+' {
            tokenizer.consume_char();
            println!("    add rax, {}", tokenizer.consume_number());
            continue;
        }
        else if tokenizer.next_char() == '-' {
            tokenizer.consume_char();
            println!("    sub rax, {}", tokenizer.consume_number());
            continue;
        }
        println!("unexpected char: {}", tokenizer.next_char());
        return;
    }

    println!("    ret");
    */

}

#[test]
fn test_func () {
    println!("=== test start ===");

    let mut tokenizer = Parser{ pos:0, input: "1+(2*3-(4*5/6+(7-8)/9))".to_string() };
    
    println!("{:?}", tokenizer.expr() );
}

#[derive(Debug)]
enum NodeKind {
    Plus, Minus, Mul, Div,
    Num(i32),
}

// Abstract syntax tree
#[derive(Debug)]
enum AST {
    Nil,
    Node{
        kind: NodeKind,
        lhs: Box<AST>,
        rhs: Box<AST>,
    }
}

#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

impl Parser {

    fn next_char(&self) -> char {
        if self.is_eof() { self.error_at(self.pos, format_args!("unexpected EOF")); }
        self.input[self.pos..].chars().next().unwrap()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // 期待された文字を読む
    fn consume(&mut self, expected: char) {
        if self.next_char() == expected {
            self.consume_char();
            return;
        }
        self.error_at(self.pos,
            format_args!("'{} 'is expected, but {} is found", expected, self.next_char()));
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.is_eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }
            return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // 単項+/-には未対応
    fn consume_number(&mut self) -> i32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });
        match s.parse::<i32>() {
            Ok(i) => { return i; },
            Err(_) => {
                self.error_at(self.pos, format_args!("number is expected"));
                return 0;
            }
        }
    }

    //エラー出力関数の作成
    fn error_at(&self, loc: usize, args: fmt::Arguments) {
        println!("{}", self.input);
        print!("{}"," ".repeat(loc));
        println!("^ ");
        print!("{}"," ".repeat(loc));
        println!("{}", args);
        println!("");

        panic!("invalid input at character: {}", loc);
    }

    // expr = mul ("+" mul | "-" mul)*
    fn expr(&mut self) -> AST {
        let mut ast = self.mul();

        while !self.is_eof() {
            match self.next_char() {
                '+' => {
                    self.consume_char();
                    ast = AST::Node{ kind: NodeKind::Plus, 
                        lhs: Box::new(ast), rhs: Box::new(self.mul()) }; },
                '-' => {
                    self.consume_char();
                    ast = AST::Node{ kind: NodeKind::Minus, 
                        lhs: Box::new(ast), rhs: Box::new(self.mul()) };
                    },
                _ => { return ast; }
            }
        }
        ast
    }

    // mul = primary ("*" primary | "/" primary)*
    fn mul(&mut self) -> AST {
        let mut ast = self.primary();

        while !self.is_eof() {
            match self.next_char() {
                '*' => {
                    self.consume_char();
                    ast = AST::Node{ kind: NodeKind::Mul, 
                        lhs: Box::new(ast), rhs: Box::new(self.primary()) };
                },
                '/' => {
                    self.consume_char();
                    ast = AST::Node{ kind: NodeKind::Div, 
                        lhs: Box::new(ast), rhs: Box::new(self.primary()) };
                },
                _ => { return ast; }
            }
        }
        ast
    }

    // primary = num | "(" expr ")"
    fn primary(&mut self) -> AST {
        
        // ( expr )
        if self.next_char() == '(' {
            self.consume('(');
            let ast = self.expr();
            self.consume(')');
            return ast;
        }
        // num
        else {
            AST::Node{ kind: NodeKind::Num(self.consume_number()), 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }
        }
    }
    
}

