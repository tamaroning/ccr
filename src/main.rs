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

    let mut tokenizer = Tokenizer { pos: 0, input: argv[1].clone(),};

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

}

#[test]
fn test_func () {
    println!("=== test start ===");

    let tokens = tokenize(String::from("1+a2+3"));
    println!("{:?}", tokens);


}

#[derive(Debug)]
enum TokenKind {
    Num(i32), // 整数
    Plus,
    Minus,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
}

#[derive(Debug)]
struct Tokenizer {
    pos: usize,
    input: String,
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer{ pos: 0, input: input };
    let mut tokens = Vec::new();

    while !tokenizer.is_eof() {
        tokenizer.consume_whitespace();
        match tokenizer.next_char() {
            '0'..='9' => tokens.push(Token{ kind: TokenKind::Num(tokenizer.consume_number()) }) ,
            '+' => {
                tokenizer.consume_char();
                tokens.push(Token{ kind: TokenKind::Plus });
            },
            '-' => {
                tokenizer.consume_char();
                tokens.push(Token{ kind: TokenKind::Minus });
            },
            _ => {
                tokenizer.error_at(tokenizer.pos, format_args!("invalid character"));
            },
        }
    }
    tokens
}

impl Tokenizer {

    fn next_char(&self) -> char {
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
        s.parse::<i32>().unwrap()
    }

    //エラー出力関数の作成 動作未確認
    fn error_at(&self, loc: usize, args: fmt::Arguments) {
        println!("{}", self.input);
        print!("{}"," ".repeat(loc));
        println!("^ ");
        print!("{}"," ".repeat(loc));
        println!("{}", args);
        println!("");

        panic!("invalid input at character: {}", loc);

    }

    /*
    fn consume_operator(&mut self) -> Token {
        match self.consume_char() {
            '+' => Token{ kind: TokenKind::Plus },
            '-' => Token{ kind: TokenKind::Minus },
            _ => panic!("invalid input"),
        }
    }
    */
}

