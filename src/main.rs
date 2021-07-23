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

    println!(".intel_syntax noprefix");
    println!(".section __TEXT,__text");
    println!(".global _main");
    println!("_main:");
    
    gen(parse(argv[1].clone()));
    
    println!("    pop rax");
    println!("    ret");
    
}

#[test]
fn test_func () {
    println!("=== test starts ===");

    let mut parser = Parser{ pos:0, input: "1+(2*3-(4*5/6+(7-8)/9))".to_string() };
    
    gen(parser.expr());

    println!("=== test finished ===");
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

fn new_node_num(val: i32) -> AST {
    AST::Node{ kind: NodeKind::Num(val), 
        lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }
}

fn gen(ast: AST) {
    match ast {
        AST::Node{ kind: NodeKind::Num(i), .. } => {
            println!("    push {}", i);
            return;
        },
        AST::Node{ kind: k, lhs: l, rhs: r } => {
            gen(*l);
            gen(*r);

            println!("    pop rdi");
            println!("    pop rax");

            match k {
                NodeKind::Plus => { println!("    add rax, rdi"); },
                NodeKind::Minus => { println!("    sub rax, rdi"); },
                NodeKind::Mul => { println!("    imul rax, rdi"); },
                NodeKind::Div => {
                    println!("    cqo");
                    println!("    idiv rdi");
                },
                _ => (),  
            };
            println!("    push rax");

            return;
        },
        _ => ()
    };
}

#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

fn parse(input: String) -> AST {
    let mut parser = Parser{ pos: 0, input: input };
    parser.consume_whitespace();
    parser.expr()
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
            format_args!("'{}' is expected, but got {}", expected, self.next_char()));
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

    // 0以上の整数を消費
    fn consume_number(&mut self) -> i32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });
        self.consume_whitespace();

        match s.parse::<i32>() {
            Ok(i) => { return i; },
            Err(_) => {
                self.error_at(self.pos, format_args!("number is expected"));
                return 0;
            }
        }
    }

    //エラー出力関数
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
    // 開始時に空白を指していることはない
    fn expr(&mut self) -> AST {
        let mut ast = self.mul();

        while !self.is_eof() {
            self.consume_whitespace();
            match self.next_char() {
                '+' => {
                    self.consume_char();
                    self.consume_whitespace();
                    ast = AST::Node{ kind: NodeKind::Plus, 
                        lhs: Box::new(ast), rhs: Box::new(self.mul()) }; },
                '-' => {
                    self.consume_char();
                    self.consume_whitespace();
                    ast = AST::Node{ kind: NodeKind::Minus, 
                        lhs: Box::new(ast), rhs: Box::new(self.mul()) };
                    },
                _ => { return ast; }
            }
        }
        ast
    }

    // mul = primary ("*" primary | "/" primary)*
    // 開始時に空白を指していることはない
    fn mul(&mut self) -> AST {
        let mut ast = self.unary();

        while !self.is_eof() {
            match self.next_char() {
                '*' => {
                    self.consume_char();
                    self.consume_whitespace();
                    ast = AST::Node{ kind: NodeKind::Mul, 
                        lhs: Box::new(ast), rhs: Box::new(self.unary()) };
                },
                '/' => {
                    self.consume_char();
                    self.consume_whitespace();
                    ast = AST::Node{ kind: NodeKind::Div, 
                        lhs: Box::new(ast), rhs: Box::new(self.unary()) };
                },
                _ => { return ast; }
            };
        }
        ast
    }

    // 単項+/-
    // 開始時に空白を指していることはない
    fn unary(&mut self) -> AST {
        match self.next_char() {
            '+' => {
                self.consume_char();
                self.consume_whitespace();
                return new_node_num(self.consume_number());
            },
            '-' => {
                self.consume_char();
                self.consume_whitespace();
                return AST::Node{
                    kind: NodeKind::Minus,
                    lhs: Box::new(new_node_num(0)),
                    rhs: Box::new(new_node_num(self.consume_number()))};
            }
            _ => { return self.primary(); }
        };

    }

    // primary = num | "(" expr ")"
    // 開始時に空白を指していることはない
    fn primary(&mut self) -> AST {
        // "(" expr ")"
        if self.next_char() == '(' {
            self.consume('(');
            self.consume_whitespace();

            let ast = self.expr();

            self.consume(')');
            self.consume_whitespace();
            
            return ast;
        }
        // num
        else {
            return new_node_num(self.consume_number());
        }
    }

}

