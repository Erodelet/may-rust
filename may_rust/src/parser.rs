use crate::parser::lexer::token::Token;
use crate::parser::lexer::Lexer;

pub mod lexer;

pub struct Parser {
    pub lexer : Lexer,
    pub token : Token,
}

impl Parser {
    pub fn new(file: &str) -> Self {
        Self {
            lexer: Lexer::new(file),
            token: Token::EOF,
        }
    }

    pub fn next_token(&mut self) {
        self.token = self.lexer.next_token()
    }

    fn accept(&mut self, t: &Token) -> bool {
        if t == &self.token{
            self.next_token();
            return true
        }
        return false
    }

    fn expect(&mut self, t:Token, caller:&str) -> bool {
        if self.accept(&t){
            return true
        }
        panic!("Token inatendu : {:?}, attendait {:?}. Erreur de syntaxe {:?}.", t, self.token, caller);
    }

    fn ident(&mut self){
        if self.accept(&Token::Identifier){
            return
        }
        panic!("Erreur de syntaxe identifier");
    }

    fn part(&mut self){
        while self.accept(&Token::Part){
            self.ident();
            self.expect(Token::Colon, "part1");
            self.ident();
            if self.accept(&Token::Lbracket){
                self.ident();
                self.expect(Token::Rbracket, "part2");
            }
            self.expect(Token::Lbrace, "part3");
            while self.accept(&Token::Bind){
                self.ident();
                self.expect(Token::To, "part4");
                self.ident();
                if self.accept(&Token::Dot){
                    self.ident();
                }
            }
            self.expect(Token::Rbrace, "part5");
        }
    }

    fn provides(&mut self){
        if self.accept(&Token::Provides) {
            self.ident();
            self.expect(Token::Colon, "provides1");
            self.ident();
            if self.accept(&Token::Equals){
                self.ident();
                self.expect(Token::Dot, "provides2");
                self.ident();
            }
            while self.accept(&Token::Provides) {
                self.ident();
                self.expect(Token::Colon, "provides1");
                self.ident();
                if self.accept(&Token::Equals){
                    self.ident();
                    self.expect(Token::Dot, "provides2");
                    self.ident();
                }
            }
            self.part();
        } else {
            panic!("Missing provides")
        }
    }

    fn requires(&mut self){
        while self.accept(&Token::Requires){
            self.ident();
            self.expect(Token::Colon, "requires");
            self.ident();
        }
        self.provides();
    }

    fn component(&mut self){
        self.expect(Token::Component, "component1");
        self.ident();
        if self.accept(&Token::Specializes){
            self.ident();
        }
        if self.accept(&Token::Lbracket){
            self.ident();
            self.expect(Token::Rbracket, "component2");
        }
        self.expect(Token::Lbrace, "component3");
        self.requires();
        self.expect(Token::Rbrace, "component4");
    }

    pub fn namespace(&mut self){
        while self.accept(&Token::Import){
            self.ident();
            while self.accept(&Token::Dot) {
                self.ident();
            }
        }
        self.expect(Token::Namespace, "namespace1");
        self.ident();
        while self.accept(&Token::Dot) {
            self.ident();
        }
        self.expect(Token::Lbrace, "namespace2");
        self.component();
        self.expect(Token::Rbrace, "namespace3");
    }
}