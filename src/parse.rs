// parse.rs

//use std::fmt;
use crate::tokenize::Token;
use crate::tokenize::TokenKind;


#[test]
fn test_parse() {

}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Plus, Minus, Mul, Div, // +,-,*,/
    Eq, Ne, Le, Lt, // ==,!=,<=,<
    Assign, // =
    Lvar{name: String, offset: usize}, // 一文字のローカル変数(変数名, rbpからのオフセット)
    Num(i32), // 整数
}

// Abstract syntax tree
#[derive(Debug, Clone)]
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
    tokens: Vec<Token>, // Tokenリスト
    pos: usize, // 現在のtokenのインデックス
}

pub fn parse(tokens: Vec<Token>) -> Vec<AST> {
    let mut parser = Parser{ pos: 0, tokens: tokens };
    parser.program()
}

impl Parser {

    // 現在のトークンを読む(読み進めない)
    fn cur_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    // 現在のトークンがEOFかどうか返す
    fn is_eof(&self) -> bool {
        match self.cur_token() {
            Token{ kind: TokenKind::Eof ,.. } => {
                true
            },
            _ => false,
        }
    }

    fn is_num(&self) -> bool {
        match self.cur_token() {
            Token{ kind: TokenKind::Num(_) ,.. } => {
                true
            },
            _ => false,
        }
    }

    // 現在のトークンを読み進めて、それを返す
    fn consume_any(&mut self) -> Token {
        let ret = self.cur_token();
        self.pos += 1;
        //println!("consumed index: {}, Token: {:?}", self.pos, ret);
        ret
    }

    // 現在のトークンが指定された文字列のreservedトークンに一致すれば、読み進めてtrueを返す
    // 一致しなければfalseを返す
    fn consume(&mut self, string: &str) -> bool {
        match self.cur_token() {
            Token{ kind: TokenKind::Reserved(t) ,.. } if t == string  => {
                self.consume_any();
                true
            },
            _ => false,
        }
    }

    // 現在のトークンは指定された文字列のreservedトークンであるに違いないので読み進める
    fn consume_expected(&mut self, string: &str) {
        if !self.consume(string) {
            panic!("unexpected token");
        }
    }

    // 現在のトークンはNumトークンであり、それを読み進めて返す
    fn consume_number(&mut self) -> i32 {
        match self.consume_any() {
            Token{ kind: TokenKind::Num(n) ,.. } => n,
            _ => {
                panic!("number is expected");
            },
        }
    }

    /*
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
    */

    // program = stmt*
    fn program(&mut self) -> Vec<AST> {
        let mut ret = Vec::new();
        //let mut i :usize = 0;
        loop {
            //println!("statement[{}]", i);
            if self.is_eof() { break; }
            ret.push(self.stmt());
            //i += 1;
        }
        ret
    }

    // stmt = expr ";"
    fn stmt(&mut self) -> AST {
        let ast = self.expr();
        self.consume_expected(";");
        ast
    }

    // expr = assign
    fn expr(&mut self) -> AST {
        self.assign()
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> AST {
        let mut ast = self.equality();
        while !self.is_eof() {
            if self.consume("=") {
                ast = AST::Node{ kind: NodeKind::Assign, lhs: Box::new(ast), rhs: Box::new(self.assign()) };
            } else {
                break;
            }
        }
        ast
    }
    
    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> AST {
        let mut ast = self.relational();
        while !self.is_eof() {
            if self.consume("==") {
                ast = AST::Node{ kind: NodeKind::Eq, lhs: Box::new(ast), rhs: Box::new(self.relational()) };
            } else if self.consume("!=") {
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
                ast = AST::Node{ kind: NodeKind::Le, lhs: Box::new(ast), rhs: Box::new(self.add()) };
            } else if self.consume("<") {
                ast = AST::Node{ kind: NodeKind::Lt, lhs: Box::new(ast), rhs: Box::new(self.add()) };
            } else if self.consume(">=") {
                ast = AST::Node{ kind: NodeKind::Le, rhs: Box::new(ast), lhs: Box::new(self.add()) };
            } else if self.consume(">") {
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
            if self.consume("+") {
                ast = AST::Node{ kind: NodeKind::Plus, 
                    lhs: Box::new(ast), rhs: Box::new(self.mul()) };
            } else if self.consume("-") {
                ast = AST::Node{ kind: NodeKind::Minus, 
                    lhs: Box::new(ast), rhs: Box::new(self.mul()) };
            } else {
                break;
            }
        }
        ast
    }


    // mul = unary ("*" unary | "/" unary)*
    // 開始時に空白を指していることはない
    fn mul(&mut self) -> AST {
        let mut ast = self.unary();

        while !self.is_eof() {
            if self.consume("*") {
                ast = AST::Node{ kind: NodeKind::Mul, 
                    lhs: Box::new(ast), rhs: Box::new(self.unary()) };
            } else if self.consume("/") {
                ast = AST::Node{ kind: NodeKind::Div, 
                    lhs: Box::new(ast), rhs: Box::new(self.unary()) };
            } else {
                break;
            }
        }
        ast
    }

    // 単項+/-
    // unary = ("+" | "-")? primary
    // 開始時に空白を指していることはない
    fn unary(&mut self) -> AST {
        if self.consume("+") {
            return self.primary();
        } else if self.consume("-") {
            return AST::Node{ kind: NodeKind::Minus,
                    lhs: Box::new(new_node_num(0)), rhs: Box::new(self.primary()) };
        } else {
            return self.primary();
        }
    }

    // primary = num | ident | "(" expr ")"
    // 開始時に空白を指していることはない
    fn primary(&mut self) -> AST {
        // "(" expr ")"
        if self.consume("(") {
            let ast = self.expr();
            self.consume_expected(")");
            return ast;
        }
        // num
        else if self.is_num() {
            return new_node_num(self.consume_number());
        }
        // ident
        else {
            return self.ident();
        }
    }

    // 一文字のローカル変数
    fn ident(&mut self) -> AST {
        match self.consume_any() {
            Token{ kind: TokenKind::Ident(name), .. } => {
                let name_chars: Vec<char> = name.chars().collect();
                return AST::Node{ kind: NodeKind::Lvar{ name: name.to_string(), offset: (name_chars[0] as usize - 'a' as usize) * 8 },
                    lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
            },
            _ => {
                panic!("variable is expected");
            }
        }
    }

}

