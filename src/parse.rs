// parse.rs

#[allow(unused_imports)]
use std::fmt;
use std::collections::HashMap;

use crate::tokenize::Token;
use crate::tokenize::TokenKind;
#[allow(unused_imports)]
use crate::tokenize::tokenize;

#[test]
fn test_parse() {
    let tokens = tokenize(String::from("int a,b;int c=a;"));
    println!("{:?}", tokens);
    let ast = parse(tokens);
    println!("{:?}", ast);
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Int, // Todo now is the same as int64_t
    Ptr(Box<TypeKind>),
}

#[derive(Debug, Clone)]
pub struct Type {
    kind: TypeKind,
    name: String,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    // --- Expression --- 
    Num(isize), // integers
    Assign, // = (assignment)
    Plus, Minus, Mul, Div, // +,-,*,/
    Eq, Ne, Le, Lt, // ==,!=,<=,<
    Deref, Addr, // *, &
    Var{ name: String, offset: usize }, // local variables (offset from rbp)
    FuncCall{ name: String, argv: Box<Vec<AST>> }, // function call

    //DeclareVar{ dec: Box<Vec<(Type, AST)>> }, // declaretion of variables (Type of var, initial assignment)

    // --- Statement ---
    ExprStmt(Box<AST>),
    Block(Box<Vec<AST>>), // {} block
    Return, // return statement, lhs used as return values
    If{ cond: Box<AST>, then: Box<AST>, els: Box<AST> }, // if([cond(expr)])[then(stmt)] else [els(stmt)]
    While{ cond: Box<AST>, proc: Box<AST> }, //while([cond(expr)]) [proc(stmt)]
    For{ a: Box<AST>, b: Box<AST>, c: Box<AST>, proc: Box<AST> }, // for([A(expr)];[B(expr)];[C(expr)]) [D(stmt)]

}

// Abstract syntax tree
#[derive(Debug, Clone)]
pub enum AST {
    Nil, 
    Node{
        kind: NodeKind, // Node kind
        // left and right side value (used only when the node is calculation)
        lhs: Box<AST>, 
        rhs: Box<AST>, 
    }
}

impl AST {
    pub fn kind(&self) -> NodeKind {
        match self.clone() {
            AST::Node{ kind: k, ..} => k,
            _ => panic!("Nil doesn't have kind"),
        }
    }
    pub fn lhs(&self) -> Box<AST> {
        match self.clone() {
            AST::Node{ lhs: l, ..} => l,
            _ => panic!("Nil doesn't have lhs"),
        }
    }
}

fn new_node_num(val: isize) -> AST {
    AST::Node{ kind: NodeKind::Num(val), 
        lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>, // Token list
    pos: usize, // current index of tokens
    offset: usize, // current stack frame size (increase by 8 when a new local var is defined)
    locals: HashMap<String, usize>, // local variables list <name, offset>
}

pub fn parse(tokens: Vec<Token>) -> Vec<AST> {
    let mut parser = Parser{ pos: 0, tokens: tokens, offset: 0, locals: HashMap::new() };
    parser.program()
}

impl Parser {
    // read the current token (don't read forward)
    fn cur_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    // check if the current token matches the token with the specified string
    fn is(&self, string: &str) -> bool {
        match self.cur_token() {
            Token{ string: t, .. } if t == string  => {
                true
            },
            _ => false,
        }
    }

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

    fn is_expr(&self) -> bool {
        if self.is("+") | self.is("-") | self.is("*") | self.is("&")  { return true; }
        return match self.cur_token() {
            Token{ kind: TokenKind::Ident(_), .. } | Token{ kind: TokenKind::Num(_), .. } => true,
            _ => false,
        };
    }

    fn is_declspec(&self) -> bool {
        if self.is("int") { return true; }
        else { return false; }
    }

    // read forward the current token and return it 
    fn consume_any(&mut self) -> Token {
        let ret = self.cur_token();
        self.pos += 1;
        //d 
        println!("consumed index: {}, Token: {:?}", self.pos, ret);
        ret
    }

    // 現在のトークンが指定された文字列のreservedトークンに一致すれば、読み進めてtrueを返す
    // 一致しなければfalseを返す
    fn consume(&mut self, string: &str) -> bool {
        match self.cur_token() {
            Token{ string: t ,.. } if t == string  => {
                self.consume_any();
                true
            },
            _ => false,
        }
    }

    // read forward the expected token
    fn expected(&mut self, string: &str) {
        if !self.consume(string) {
            self.error_at("unexpected token");
        }
    }

    // 現在のトークンはNumトークンであり、それを読み進めて返す
    fn consume_number(&mut self) -> isize {
        match self.consume_any() {
            Token{ kind: TokenKind::Num(n) ,.. } => n,
            _ => {
                self.error_at("number is expected");
                0
            },
        }
    }

    fn error_at(&self, string: &str) {
        println!("{}", string);
        panic!("error! pos: {}, token: {:?}", self.pos, self.cur_token());
    }


    // ----- Description of grammar by EBNF -----

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
    //      | declararion ";"
    //      | "{" stmt* "}"
    //      | "return" expr ";"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    // Todo declarationを式として評価したい(ex: for(int i;;) )
    fn stmt(&mut self) -> AST {
        // "return" expr ";" 
        if self.consume("return") {
            let ast = AST::Node{ kind: NodeKind::Return, lhs: Box::new(self.expr()), rhs: Box::new(AST::Nil),  };
            self.expected(";");
            return ast;
        }
        // "if" "(" expr ")" stmt ("else" stmt)?
        else if self.consume("if") {
            self.expected("(");
            let cond = self.expr();
            self.expected(")");
            let then = self.stmt();
            let mut els = AST::Nil;
            if self.consume("else") {
                els = self.stmt();
            }
            return AST::Node{ kind: NodeKind::If{ cond: Box::new(cond), then: Box::new(then), els: Box::new(els)}, 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "while" "(" expr ")" stmt
        else if self.consume("while") {
            self.expected("(");
            let cond = self.expr();
            self.expected(")");
            let proc = self.stmt();
            return AST::Node{ kind: NodeKind::For{ a: Box::new(AST::Nil),
                b: Box::new(cond), c: Box::new(AST::Nil), proc: Box::new(proc) }, 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
        else if self.consume("for") {
            self.expected("(");
            let expr_a = self.expr();
            self.consume(";");
            let expr_b = self.expr();
            self.consume(";");
            let expr_c = self.expr();
            self.expected(")");
            let proc = self.stmt();
            return AST::Node{ kind: NodeKind::For{ a: Box::new(expr_a), b: Box::new(expr_b), c: Box::new(expr_c), 
                proc: Box::new(proc) }, lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "{" stmt* "}"
        else if self.consume("{") {
            let mut vec = Vec::new();
            while !self.consume("}") {
                vec.push(self.stmt());
            }
            return AST::Node{ kind: NodeKind::Block(Box::new(vec)), 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        else if self.is_declspec() {
            let ast = self.declaration();
            self.expected(";");
            return ast;;
        }
        // expr ";"
        else {
            let ast = self.expr();
            self.expected(";");
            return AST::Node{ kind: NodeKind::ExprStmt(Box::new(ast)), lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
    }

    // expr = assign
    //      | blank expression (OK only if the current token matches to ";") 
    fn expr(&mut self) -> AST {
        if self.is(";") { return AST::Nil; }
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

    // unary = ("+" | "-" | "*" | "&")? unary
    //       | primary
    fn unary(&mut self) -> AST {
        if self.consume("+") {
            return self.unary();
        } else if self.consume("-") {
            return AST::Node{ kind: NodeKind::Minus,
                    lhs: Box::new(new_node_num(0)), rhs: Box::new(self.unary()) };
        } else if self.consume("*") {
            return AST::Node{ kind: NodeKind::Deref,
                    lhs: Box::new(self.unary()), rhs: Box::new(AST::Nil) };
        }  else if self.consume("&") {
            return AST::Node{ kind: NodeKind::Addr,
                    lhs: Box::new(self.unary()), rhs: Box::new(AST::Nil) };
        } else {
            return self.primary();
        }
    }

    // primary = num | "(" expr ")"
    //         | ident (variables and function calls)
    fn primary(&mut self) -> AST {
        // "(" expr ")"
        if self.consume("(") {

            let ast = self.expr();
            self.expected(")");
            return ast;
        }
        // num
        else if self.is_num() {
            return new_node_num(self.consume_number());
        }
        // ident (variables and function calls)
        else {
            return self.ident();
        }
    }

    // ident = ident (variables)
    //       | ident "(" (expr ",")* ")" (function calls)
    fn ident(&mut self) -> AST {

        let ident_name = match self.consume_any() {
            Token{ kind: TokenKind::Ident(s), .. } => s,
            _ => panic!("unexpected token"),
        };
        
        // func_name"(" (expr ",")* ")" (function call)
        if self.consume("(") {
            let mut argv: Vec<AST>= Vec::new();

            if self.is_expr() { argv.push(self.expr()); }
            loop {
                if !self.consume(",") { break; }
                if !self.is_expr() { break; }
                argv.push(self.expr()); 
            }

            self.expected(")");
            return AST::Node{ kind: NodeKind::FuncCall{ name: ident_name.clone(), argv: Box::new(argv) }, 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }

        // variables
        match self.locals.get(&ident_name) {
            // variable names are already registered
            Some(ofs) => {
                return AST::Node{ kind: NodeKind::Var{ name: ident_name.clone(), offset: *ofs },
                    lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
            },
            // not registered
            None => {
                panic!("{} is not defined", ident_name.clone());
            },
        };
            
    }

    // declspec = "int"
    fn declspec(&mut self) -> TypeKind {
        if self.consume("int") {
            return TypeKind::Int;
        }
        else {
            self.error_at("unexpected type");
            TypeKind::Int
        }
    }

    // declarator = "*"* ident
    // Todo 現在はint型にしか対応していない
    fn declarator(&mut self, tk: TypeKind) -> Type {

        while self.consume("*") {
            panic!("pointer type is not implemented");
            //tk = TypeKind::Ptr(Box::new(tk));
        }
        let ident_name = match self.consume_any() {
            Token{ kind: TokenKind::Ident(s), .. } => s,
            _ => panic!("expected a variable name"),
        };
        Type{ name: ident_name, kind: tk }
    }

    // declaration = declspec (declarator ("=" expr)? ("," declarator ("=" expr)?)*)? ";"
    fn declaration(&mut self) -> AST {
        let mut inits: Vec<AST> = Vec::new();
        let type_kind = self.declspec();
        
        while {
            let declarator = self.declarator(type_kind.clone());
            let var_name = match declarator.clone() {
                Type{ name: s, .. } => s,
            };

            // オフセットの割当て
            let offset = match self.locals.get(&var_name) {
                // variable names are already registered
                Some(&ofs) => {
                    self.locals.insert(var_name.clone(), ofs);
                    ofs
                },
                // not registered
                None => {
                    self.locals.insert(var_name.clone(), self.offset);
                    self.offset += 8;
                    self.offset - 8
                },
            };

            if self.consume("=") {
                let init = AST::Node{ kind: NodeKind::Assign,
                    lhs: Box::new(AST::Node{ kind: NodeKind::Var{ name: var_name.clone(), offset: offset },
                    lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }),
                    rhs: Box::new(self.expr()) };
                inits.push(AST::Node{ kind: NodeKind::ExprStmt(Box::new(init)), lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) });
        
            }

            self.consume(",") // loop only while this is met
        } {}

        AST::Node{ kind: NodeKind::Block(Box::new(inits)), 
            lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) }
    }        

}

