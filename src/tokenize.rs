// tokenize.rs

#[allow(unused_imports)]
use std::fmt;

const KEYWORD: [&'static str; 7] = ["return", "if", "else", "for", "while", "int", "void"];

#[test]
fn test_tokenize() {
    let tokens = tokenize(String::from("if(a)a =1;else a = 1;"));
    println!("{:?}", tokens);

}

#[derive(Debug, Clone,PartialEq)]
pub enum TokenKind {
    Reserved, // keywords or punctuators
    Num(isize), // integer literals(value)
    Ident, // identifiers(name) function name and variable name
    Keyword, // Keywords (return, if, ...)
    Eof, // end of the tokens
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind, // Token kind
    pos: usize, // start positon of the token
    pub string: String, // token string
}

#[derive(Debug)]
struct Tokenizer {
    pos: usize,
    input: String,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut tokenizer = Tokenizer{ pos: 0, input: input };
    
    while !tokenizer.is_eof() {

        tokenizer.read_whitespace();
        if tokenizer.is_eof() { break; }

        // numeric literals
        match tokenizer.next_char() {
            '0'..='9' => {
                tokens.push(Token{ kind: TokenKind::Num(tokenizer.read_number()), pos: tokenizer.pos, string: String::new() });
                continue;
            },
            _ => (),
        };

        // keywords (reserved words)
        if tokenizer.is_keyword() {
            tokens.push(tokenizer.read_keyword());                
            continue;
        }

        // identifiers 
        if tokenizer.is_al()  {
            let ident = tokenizer.read_ident();
            tokens.push(Token{ kind: TokenKind::Ident, pos: tokenizer.pos, string: ident });
            continue;
        }

        // punctuators
        if tokenizer.starts_with("==") || tokenizer.starts_with("!=") ||
                    tokenizer.starts_with("<=") || tokenizer.starts_with(">=") {
            let punc = tokenizer.read_nchars(2);
            tokens.push(Token{ kind: TokenKind::Reserved,
                pos: tokenizer.pos, string: punc });
            continue;
        }
        match tokenizer.next_char() {
            '!'|'"'|'#'|'$'|'%'|'&'|'('|')'|'*'|'+'|','|'-'|'.'|'/'|':'|';'|'<'|'='|
                    '>'|'?'|'@'|'['|'\\'|']'|'^'|'_'|'`'|'{'|'|'|'}'|'~' => {
                    let punc = tokenizer.read_nchars(1);
                tokens.push(Token{ kind: TokenKind::Reserved, 
                    pos: tokenizer.pos, string: punc });
                continue;
            },
            _ => (),
        };
        tokenizer.error_at(&format!("invalid token"));
    }
    tokens.push(Token{ kind: TokenKind::Eof, pos: tokenizer.pos, string: String::new() });
    tokens
}

impl Tokenizer {

    // read the next character
    fn next_char(&self) -> char {
        if self.is_eof() { self.error_at(&format!("unexpected EOF")); }
        self.input[self.pos..].chars().next().unwrap()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn is_al(&self) -> bool {
        match self.next_char() {
            'a'..='z'|'A'..='Z'|'_' => true,
            _ => false,
        }
    }

    // 先頭の文字がトークンを構成する文字(英数字or_)かを返す
    fn is_alnum(&self) -> bool {
        match self.next_char() {
            'a'..='z'|'A'..='Z'|'0'..='9'|'_' => true,
            _ => false,
        }
    }

    // check if the first string matches to the specified string
    fn is_keyword(&mut self) -> bool {
        if !self.is_al(){ return false; }
        for i in 0..KEYWORD.len() {
            if self.starts_with(KEYWORD[i]){ return true; }
        }
        false
    }

    // read forward one character
    fn read_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // read forward n characters
    fn read_nchars(&mut self, n: usize) -> String {
        let mut chars = Vec::new();
        for _ in 0..n {
            chars.push(self.read_char());
        }
        chars.iter().collect()
    }

    // posの指す位置がが文字列で始まってるかを返す
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // read forward while the condition is satisfied
    fn read_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.is_eof() && test(self.next_char()) {
                result.push(self.read_char());
            }
            return result;
    }

    // read forward whitespaces and LF
    fn read_whitespace(&mut self) {
        self.read_while(char::is_whitespace);
    }

    // read forward non-negative integer
    fn read_number(&mut self) -> isize {
        let s = self.read_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });
        match s.parse::<isize>() {
            Ok(i) => { return i; },
            Err(_) => {
                self.error_at(&format!("invalid number"));
                return 0;
            }
        };
    }

    fn read_ident(&mut self) -> String {
        if !self.is_al() { panic!("variable name must begin with alphabet or underscore"); }
        let s = self.read_while(|c| match c {
            'A'..='Z'|'a'..='z'|'0'..='9'|'_' => true,
            _ => false,
        });
        s
    }

    // read forward keywords
    // Todo return4;なども正しい入力と見做されることに注意
    fn read_keyword(&mut self) -> Token {
        for i in 0..KEYWORD.len() {
            if self.starts_with(KEYWORD[i]) {
                let keyword = self.read_nchars(KEYWORD[i].len());
                return Token{ kind: TokenKind::Keyword, pos: self.pos, string: keyword };
            }
        }
        self.error_at(&format!("keyword is expected"));
        panic!("");
    }

    fn error_at(&self, string: &str) {
        println!("{}", self.input);
        print!("{}"," ".repeat(self.pos));
        println!("^ ");
        print!("{}"," ".repeat(self.pos));
        println!("{}", string);
        println!("");

        panic!("invalid input at character: {}", self.pos);
    }
}