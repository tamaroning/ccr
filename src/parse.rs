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
    let tokens = tokenize(String::from("Foo(+1,a,b);"));
    println!("{:?}", tokens);
    let ast = parse(tokens);
    println!("{:?}", ast);
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Int,
    Ptr,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Num(isize), // integer
    Assign, // = assignment
    Plus, Minus, Mul, Div, // +,-,*,/
    Eq, Ne, Le, Lt, // ==,!=,<=,<
    Deref, Addr, // *, &
    Lvar{ name: String, offset: usize }, // local variables(name, offset from rbp)
    FuncCall{ name: String, argv: Box<Vec<AST>> }, // fucntion call

    DefineLvar{ name: String, ty: TypeKind },
    Return, // return 戻り値はlhsを使う
    If{ cond: Box<AST>, then: Box<AST>, els: Box<AST> }, // if([cond(expr)])[then(stmt)] else [els(stmt)]
    While{ cond: Box<AST>, proc: Box<AST> }, //while([cond(expr)]) [proc(stmt)]
    For{ a: Box<AST>, b: Box<AST>, c: Box<AST>, proc: Box<AST> }, // for([A(expr)];[B(expr)];[C(expr)]) [D(stmt)]
    Block(Box<Vec<AST>>), // {stmt*}ブロック stmtのVecをもつ

    Int, // int64_t
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

// isizeからNumノードを作成する
fn new_node_num(val: isize) -> AST {
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

    // 現在のトークンが指定された文字列のトークンに一致するか判定する
    fn is(&self, string: &str) -> bool {
        match self.cur_token() {
            Token{ string: t, .. } if t == string  => {
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

    fn is_expr(&self) -> bool {
        if self.is("+") | self.is("-") { return true; }
        return match self.cur_token() {
            Token{ kind: TokenKind::Ident(_), .. } | Token{ kind: TokenKind::Num(_), .. } => true,
            _ => false,
        };
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
            Token{ string: t ,.. } if t == string  => {
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
    fn consume_number(&mut self) -> isize {
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
    //      | declararion ";"
    //      | "{" stmt* "}"
    //      | "return" expr ";"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    fn stmt(&mut self) -> AST {
        // "return" expr ";" 
        if self.consume("return") {
            let ast = AST::Node{ kind: NodeKind::Return, lhs: Box::new(self.expr()), rhs: Box::new(AST::Nil),  };
            self.consume_expected(";");
            return ast;
        }
        // "if" "(" expr ")" stmt ("else" stmt)?
        else if self.consume("if") {
            self.consume_expected("(");
            let cond = self.expr();
            self.consume_expected(")");
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
            self.consume_expected("(");
            let cond = self.expr();
            self.consume_expected(")");
            let proc = self.stmt();
            return AST::Node{ kind: NodeKind::While{ cond: Box::new(cond), proc: Box::new(proc) }, 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }
        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
        else if self.consume("for") {
            self.consume_expected("(");
            let expr_a = self.expr();
            self.consume(";");
            let expr_b = self.expr();
            self.consume(";");
            let expr_c = self.expr();
            self.consume_expected(")");
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
    //         | ident ローカル変数
    //         | ident "(" ")" 引数なし関数呼び出し 
    fn primary(&mut self) -> AST {
        // "(" expr ")"
        if self.consume("(") {
            // exprに対応するのは Num,Reserved("("),Lvar,FuncCall
            let ast = self.expr();
            self.consume_expected(")");
            return ast;
        }
        // num
        else if self.is_num() {
            return new_node_num(self.consume_number());
        }
        // ident 関数名または変数名
        else {
            return self.ident();
        }
    }

    // ident : function name or local variable name
    fn ident(&mut self) -> AST {

        let ident_name = match self.consume_any() {
            Token{ kind: TokenKind::Ident(s), .. } => s,
            _ => panic!("unexpected token"),
        };
        
        // 関数呼び出し
        // func_name"(" (expr ",")* ")"
        if self.consume("(") {
            let mut argv: Vec<AST>= Vec::new();
            
            if self.is_expr() { argv.push(self.expr()); }
            loop {
                if !self.consume(",") { break; }
                if !self.is_expr() { break; }
                argv.push(self.expr()); 
            }

            self.consume_expected(")");
            return AST::Node{ kind: NodeKind::FuncCall{ name: ident_name.clone(), argv: Box::new(argv) }, 
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
        }

        // ローカル変数
        match self.locals.get(&ident_name) {
            // 変数名がすでに登録済み
            Some(ofs) => {
                return AST::Node{ kind: NodeKind::Lvar{ name: ident_name.clone(), offset: *ofs },
                    lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
            },
            // 変数名が未登録
            None => {
                self.locals.insert(ident_name.clone(), self.offset);
                let ret = AST::Node{ kind: NodeKind::Lvar{ name: ident_name.clone(), offset: self.offset },
                lhs: Box::new(AST::Nil), rhs: Box::new(AST::Nil) };
                //d println!("{} offset: {}", var_name, self.offset);
                self.offset += 8;
                return ret;
            },
        };
            
    }

    // declspec = "int"
    fn declspec(&mut self) -> TypeKind {
        if self.consume("int") {
            return TypeKind::Int;
        }
        else {
            panic!("unexpected type")
        }
    }

    // declarator = "*"* ident
    /*
    fn declarator(&mut self) -> TypeKind {
        if self.consume("*") {
            panic!("* is not implemented");
        } else {
            self.ident()
        }
    }
    */

    // declaration = declspec (declarator ("=" expr)? ("," declarator ("=" expr)?)*)? ";"

}

