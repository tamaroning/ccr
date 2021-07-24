// parse.rs

use std::fmt;

#[test]
fn test_parse() {
    let parser = Parser{ pos: 0, input: String::from("Hello!") };
    println!("{:?}", parser.starts_with("Hell"));

}

#[derive(Debug)]
pub enum NodeKind {
    Plus, Minus, Mul, Div, // +,-,*,/
    Eq, Ne, Le, Lt, // ==,!=,<=,<
    Num(i32),
}

// Abstract syntax tree
#[derive(Debug)]
pub enum AST {
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

#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

pub fn parse(input: String) -> AST {
    let mut parser = Parser{ pos: 0, input: input };
    parser.consume_whitespace();
    parser.expr()
}

impl Parser {

    // 先頭の一文字にアクセスする
    fn next_char(&self) -> char {
        if self.is_eof() { self.error_at(self.pos, format_args!("unexpected EOF")); }
        self.input[self.pos..].chars().next().unwrap()
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // 1文字読み進める
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // posの指す位置がが文字列で始まってるかを返す
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // 期待された文字列で始まっているならば、消費してtrueを返す
    // そうでなければfalseを返す
    fn consume(&mut self, s: &str) -> bool {
        if self.starts_with(s) {
            for _ in 0..s.len() { self.consume_char(); }
            return true;
        }
        else {
            return false;
        }
    }

    // posの指す位置から期待された文字列を消費する
    // 期待されてない文字列が発見されたらpanicを起こす
    fn consume_expected(&mut self, expected: &str) {
        if !self.consume(expected) {
            self.error_at(self.pos,
            format_args!("'{}' is expected, but got {}", expected, self.next_char()));
            
        }
    }

    // 条件が満たされる間だけ文字を消費し続ける
    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.is_eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }
            return result;
    }

    // 空白を消費する
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // 0以上の整数を消費する
    fn consume_number(&mut self) -> i32 {
        let s = self.consume_while(|c| match c {
            '0'..='9' => true,
            _ => false,
        });
        self.consume_whitespace(); //後ろに続く空白は消費して良い

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

    // expr = equality
    fn expr(&mut self) -> AST {
        self.equality()
    }
    
    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> AST {
        let mut ast = self.relational();
        while !self.is_eof() {
            if self.consume("==") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Eq, lhs: Box::new(ast), rhs: Box::new(self.relational()) };
            } else if self.consume("!=") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Ne, lhs: Box::new(ast), rhs: Box::new(self.relational()) };
            } else {
                break;
            }
        }
        ast
    }
    
    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> AST {
        let mut ast = self.add();

        while !self.is_eof() {
            if self.consume("<=") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Le, lhs: Box::new(ast), rhs: Box::new(self.add()) };
            } else if self.consume("<") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Lt, lhs: Box::new(ast), rhs: Box::new(self.add()) };
            } else if self.consume(">=") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Le, rhs: Box::new(ast), lhs: Box::new(self.add()) };
            } else if self.consume(">") {
                self.consume_whitespace();
                ast = AST::Node{ kind: NodeKind::Lt, rhs: Box::new(ast), lhs: Box::new(self.add()) };
            } else {
                break;
            }
        }
        ast
    }
    
    // add = mul ("+" mul | "-" mul)*
    // 開始時に空白を指していることはない
    fn add(&mut self) -> AST {
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
                _ => { break; }
            };
        }
        ast
    }


    // mul = unary ("*" unary | "/" unary)*
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
                _ => { break; }
            };
        }
        ast
    }

    // 単項+/-
    // unary = ("+" | "-")? primary
    // 開始時に空白を指していることはない
    fn unary(&mut self) -> AST {
        match self.next_char() {
            '+' => {
                self.consume_char();
                self.consume_whitespace();
                return self.primary();
            },
            '-' => {
                self.consume_char();
                self.consume_whitespace();
                return AST::Node{
                    kind: NodeKind::Minus,
                    lhs: Box::new(new_node_num(0)),
                    rhs: Box::new(self.primary()) };
            }
            _ => { return self.primary(); }
        };
    }

    // primary = num | "(" expr ")"
    // 開始時に空白を指していることはない
    fn primary(&mut self) -> AST {
        // "(" expr ")"
        if self.next_char() == '(' {
            self.consume_expected("(");
            self.consume_whitespace();

            let ast = self.expr();

            self.consume_expected(")");
            self.consume_whitespace();
            
            return ast;
        }
        // num
        else {
            return new_node_num(self.consume_number());
        }
    }

}

