use super::ast::Ast;
use super::lexer::Lexer;
use super::token::{JavaTokenExtension, Token};
use crate::modules::common::token::CommonToken;

pub struct Parser {
    pub lexer: Lexer,
    pub token: Token,
}

impl Parser {
    pub fn new(file: &str) -> Self {
        Self {
            lexer: Lexer::new(file),
            token: Token::Common(CommonToken::EOF),
        }
    }

    pub fn next_token(&mut self) {
        self.token = self.lexer.next_token()
    }

    fn accept(&mut self, token: &Token) -> bool {
        if token == &self.token {
            self.next_token();
            return true;
        }
        false
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
            Token::Common(CommonToken::Identifier(name)) => {
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
        if self.accept(&Token::Extended(JavaTokenExtension::Public)) {
            Some("public".to_string())
        } else {
            None
        }
    }

    fn function(&mut self) -> Ast {
        let access = self.access();

        let type_name = self.ident();
        let name = self.ident();

        self.expect(
            Token::Common(CommonToken::Lparentheses),
            "after function name",
        );
        self.expect(
            Token::Common(CommonToken::Rparentheses),
            "after function parameter list",
        );
        self.expect(Token::Common(CommonToken::Lbrace), "before function body");
        self.expect(Token::Common(CommonToken::Rbrace), "after function body");

        Ast::Function {
            name,
            type_name,
            access,
            body: None,
        }
    }

    pub fn class(&mut self) -> Ast {
        let mut nodes = Vec::new();

        while self.accept(&Token::Extended(JavaTokenExtension::Package)) {
            nodes.push(Ast::Package { name: self.ident() });
            self.expect(
                Token::Common(CommonToken::Semicolon),
                "after package declaration",
            );
        }

        let access = self.access();

        self.expect(
            Token::Extended(JavaTokenExtension::Class),
            "before class name",
        );
        let name = self.ident();
        self.expect(Token::Common(CommonToken::Lbrace), "before class body");
        let body = self.function();
        self.expect(Token::Common(CommonToken::Rbrace), "after class body");

        nodes.push(Ast::Class {
            name,
            access,
            body: Box::new(body),
        });

        Ast::SEQ(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_java_using_shared_and_java_tokens() {
        let mut parser = Parser::new("package ex1; public class Start { public void go() {} }");
        parser.next_token();

        assert_eq!(
            parser.class(),
            Ast::SEQ(vec![
                Ast::Package {
                    name: String::from("ex1"),
                },
                Ast::Class {
                    name: String::from("Start"),
                    access: Some(String::from("public")),
                    body: Box::new(Ast::Function {
                        name: String::from("go"),
                        type_name: String::from("void"),
                        access: Some(String::from("public")),
                        body: None,
                    }),
                },
            ])
        );
    }
}
