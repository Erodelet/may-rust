use crate::ast::Ast;
use crate::parser::lexer::Lexer;
use crate::parser::lexer::token::Token;

pub mod lexer;

pub struct Parser {
    pub lexer: Lexer,
    pub token: Token,
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
        if t == &self.token {
            self.next_token();
            return true;
        }
        return false;
    }

    fn expect(&mut self, expected: Token, context: &str) {
        if self.accept(&expected) {
            return;
        }

        panic!(
            "Syntax error {context}: found {:?}, expected {:?}.",
            self.token, expected
        );
    }

    fn ident(&mut self) -> String {
        match &self.token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                name
            }
            _ => panic!(
                "Token inatendu : {:?}, attendait un identifiant.",
                self.token
            ),
        }
    }

    fn access(&mut self) -> Option<String> {
        if self.accept(&Token::Public) {
            Some("public".to_string())
        } else {
            None
        }
    }

    fn function(&mut self) -> Ast {
        let access = self.access();

        let type_name = self.ident();
        let name = self.ident();

        self.expect(Token::Lparentheses, "after function name");
        self.expect(Token::Rparentheses, "after function parameter list");
        self.expect(Token::Lbrace, "before function body");
        self.expect(Token::Rbrace, "after function body");

        Ast::Function {
            name,
            type_name,
            access,
            body: None,
        }
    }

    pub fn class(&mut self) -> Ast {
        let mut nodes = Vec::new();

        while self.accept(&Token::Package) {
            nodes.push(Ast::Package { name: self.ident() });
            self.expect(Token::Semicolon, "after package declaration");
        }

        let access = self.access();

        self.expect(Token::Class, "before class name");
        let name = self.ident();
        self.expect(Token::Lbrace, "before class body");
        let body = self.function();
        self.expect(Token::Rbrace, "after class body");

        nodes.push(Ast::Class {
            name,
            access,
            body: Box::new(body),
        });

        Ast::SEQ(nodes)
    }
}
