// tokenize.rs

use std::fmt;

#[test]
fn test_tokenize() {
    let tokens = tokenize(String::from("if(a)a =1;else a = 1;"));
    println!("{:?}", tokens);

}

#[derive(Debug, Clone,PartialEq)]
pub enum TokenKind {
    Reserved(String), // keywords or punctuators
    Num(i32), // integer literals(value)
    Ident(String), // identifiers(name) function name and variable name
    Keyword(String), // returnやifなどの予約語
    Eof, // end of the 
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind, // トークンの種類
    pos: usize, // トークンの開始位置
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
                tokens.push(Token{ kind: TokenKind::Num(tokenizer.read_number()), pos: tokenizer.pos });
                continue;
            },
            _ => (),
        };

        // 予約語
        if tokenizer.is_keyword() {
            tokens.push(tokenizer.read_keyword());                
            continue;
        }

        // identifiers (変数名)
        if tokenizer.is_al()  {
            // 変数名
            tokens.push(Token{ kind: TokenKind::Ident(tokenizer.read_ident()), pos: tokenizer.pos });
            continue;
        }


        // punctuators
        if tokenizer.starts_with("==") || tokenizer.starts_with("!=") ||
                    tokenizer.starts_with("<=") || tokenizer.starts_with(">=") {
            tokens.push(Token{ kind: TokenKind::Reserved(tokenizer.read_nchars(2))
                    , pos: tokenizer.pos });
            continue;
        }
        match tokenizer.next_char() {
            '!'|'"'|'#'|'$'|'%'|'&'|'('|')'|'*'|'+'|','|'-'|'.'|'/'|':'|';'|'<'|'='|
                    '>'|'?'|'@'|'['|'\\'|']'|'^'|'_'|'`'|'{'|'|'|'}'|'~' => {
                tokens.push(Token{ kind: TokenKind::Reserved(tokenizer.read_nchars(1))
                        , pos: tokenizer.pos });
                continue;
            },
            _ => (),
        };

        // invalid tokens
        tokenizer.error_at(tokenizer.pos, format_args!("invalid token"));
    }
    tokens.push(Token{ kind: TokenKind::Eof, pos: tokenizer.pos });
    tokens
}

impl Tokenizer {

    // 先頭の文字にアクセスする
    fn next_char(&self) -> char {
        if self.is_eof() { self.error_at(self.pos, format_args!("unexpected EOF")); }
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

    // 先頭の文字列がkeywordかどうか返す
    fn is_keyword(&mut self) -> bool {
        if !self.is_al(){ return false; }
        if self.starts_with("return")|self.starts_with("if")|self.starts_with("else")|
                self.starts_with("while")|self.starts_with("for"){
            return true;
        }
        false
    }

    // 1文字読み進める
    fn read_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // n文字読み進める
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

    // 条件が満たされる間だけ文字を読む
    fn read_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.is_eof() && test(self.next_char()) {
                result.push(self.read_char());
            }
            return result;
    }

    // 空白と改行を読む
    fn read_whitespace(&mut self) {
        self.read_while(char::is_whitespace);
    }

    // 非負整数を読む
    fn read_number(&mut self) -> i32 {
        let s = self.read_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });

        match s.parse::<i32>() {
            Ok(i) => { return i; },
            Err(_) => {
                self.error_at(self.pos, format_args!("invalid number"));
                return 0;
            }
        };
    }

    // ローカル変数の文字列を読む
    fn read_ident(&mut self) -> String {
        if !self.is_al() { panic!("variable name must begin with alphabet or underscore"); }
        let s = self.read_while(|c| match c {
            'A'..='Z'|'a'..='z'|'0'..='9'|'_' => true,
            _ => false,
        });
        s
    }

    // 予約語を読む
    // return4;なども正しい入力と見做されることに注意
    fn read_keyword(&mut self) -> Token {
        let keyword;
        if self.starts_with("return"){ keyword = self.read_nchars(6); }
        else if self.starts_with("if"){ keyword = self.read_nchars(2); }
        else if self.starts_with("else"){ keyword = self.read_nchars(4); }
        else if self.starts_with("while"){ keyword = self.read_nchars(5); }
        else if self.starts_with("for"){ keyword = self.read_nchars(3); }
        else { panic!("keyword is expected"); }
        
        return Token{ kind: TokenKind::Keyword(keyword), pos: self.pos };
    }

    //tokenize時のエラーを出力する
    fn error_at(&self, loc: usize, args: fmt::Arguments) {
        println!("{}", self.input);
        print!("{}"," ".repeat(loc));
        println!("^ ");
        print!("{}"," ".repeat(loc));
        println!("{}", args);
        println!("");

        panic!("invalid input at character: {}", loc);
    }
}