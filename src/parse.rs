// parse.rs

use std::fmt;
use std::collections::HashMap;

use crate::tokenize::Token;
use crate::tokenize::TokenKind;
use crate::tokenize::tokenize;


#[test]
fn test_parse() {
    let tokens = tokenize(String::from("for(A;B;C) D;"));
    println!("{:?}", tokens);
    let ast = parse(tokens);
    println!("{:?}", ast);
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Plus, Minus, Mul, Div, // +,-,*,/
    Eq, Ne, Le, Lt, // ==,!=,<=,<
    Assign, // =
    Lvar{name: String, offset: usize}, // 一文字のローカル変数(変数名, rbpからのオフセット)
    Num(i32), // 整数

    Return, // return文 戻り値はlhsを使う
    If(Box<AST>, Box<AST>, Box<AST>), // if([cond])[then] else [else]　cond(expr), then(stmt), else(stmt)
    While(Box<AST>, Box<AST>), //while([cond]) [proc]
    For(Box<AST>, Box<AST>, Box<AST>, Box<AST>) // for文 for([A];[B];[C]) [D]
}

// Abstract syntax tree
#[derive(Debug, Clone)]
pub enum AST {
    Nil, 
    Node{
        kind: NodeKind, // ノードの種類
        lhs: Box<AST>, // 左辺値
        rhs: Box<AST>, // 右辺値
    }
}

// i32からNumノードを作成する
fn new_node_num(val: i32) -> AST {
    AST::Node{ kind: NodeKind::Num(val), 
        lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>, // Tokenリスト
    pos: usize, // 現在のtokenのインデックス
    offset: usize,
    locals: HashMap<String, usize>, // ローカル変数<name, offset>
}

pub fn parse(tokens: Vec<Token>) -> Vec<AST> {
    let mut parser = Parser{ pos: 0, tokens: tokens, offset: 0, locals: HashMap::new() };
    parser.program()
}

impl Parser {
    // 現在のトークンを読む(読み進めない)
    fn cur_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    // 現在のトークンがEOFか判定する
    fn is_eof(&self) -> bool {
        match self.cur_token() {
            Token{ kind: TokenKind::Eof ,.. } => {
                true
            },
            _ => false,
        }
    }

    // 現在のトークンがNumか判定する
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
        //d println!("consumed index: {}, Token: {:?}", self.pos, ret);
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

    // consumeの引数をTokenKindに置き換えたもの
    fn consume_keyword(&mut self, string: &str) -> bool {
        match self.cur_token() {
            Token{ kind: TokenKind::Keyword(s), ..} if s == string.to_string() => {
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

    // EBNFによる文法の生成
    // program = stmt*
    fn program(&mut self) -> Vec<AST> {
        let mut ret = Vec::new();
        loop {
            //println!("statement[{}]", i);
            if self.is_eof() { break; }
            ret.push(self.stmt());
        }
        ret
    }

    // stmt = expr ";" 
    //      | "return" expr ";"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    fn stmt(&mut self) -> AST {
        // "return" expr ";" 
        if self.consume_keyword("return") {
            let ast = AST::Node{ kind: NodeKind::Return, lhs: Box::new(self.expr()), rhs: Box::new(AST::Nil),  };
            self.consume_expected(";");
            return ast;
        }
        // "if" "(" expr ")" stmt ("else" stmt)?
        else if self.consume_keyword("if") {
            self.consume_expected("(");
            let cond = self.expr();
            self.consume_expected(")");
            let then = self.stmt();
            let mut els = AST::Nil;
            if self.consume_keyword("else") {
                els = self.stmt();
            }
            return AST::Node{ kind: NodeKind::If(Box::new(cond), Box::new(then), Box::new(els)), 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "while" "(" expr ")" stmt
        else if self.consume_keyword("while") {
            self.consume_expected("(");
            let cond = self.expr();
            self.consume_expected(")");
            let proc = self.stmt();
            return AST::Node{ kind: NodeKind::While(Box::new(cond), Box::new(proc)), 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
        else if self.consume_keyword("for") {
            self.consume_expected("(");
            let expr_a = self.expr();
            self.consume(";");
            let expr_b = self.expr();
            self.consume(";");
            let expr_c = self.expr();
            self.consume_expected(")");
            let proc = self.stmt();
            return AST::Node{ 
                kind: NodeKind::For(Box::new(expr_a), Box::new(expr_b), Box::new(expr_c), Box::new(proc)), 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // expr ";"
        else {
            let ast = self.expr();
            self.consume_expected(";");
            return ast;
        }
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

    // unary = ("+" | "-")? primary
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
            Token{ kind: TokenKind::Ident(var_name), .. } => {
                match self.locals.get(&var_name) {
                    // 変数名がすでに登録済み
                    Some(ofs) => {
                        return AST::Node{ kind: NodeKind::Lvar{ name: var_name.clone(), offset: *ofs },
                    lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
                    },
                    // 変数名が未登録
                    None => {
                        self.locals.insert(var_name.clone(), self.offset);
                        let ret = AST::Node{ kind: NodeKind::Lvar{ name: var_name.clone(), offset: self.offset },
                        lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
                        //d println!("{} offset: {}", var_name, self.offset);
                        self.offset += 8;
                        return ret;
                    },
                };
            },
            _ => {
                panic!("variable is expected");
            }
        }
    }

}

