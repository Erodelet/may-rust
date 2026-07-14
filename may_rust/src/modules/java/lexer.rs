use super::token::{JavaTokenExtension, Token};
use crate::modules::common::lexer::CharReader;
use crate::modules::common::token::{CommonToken, Token as SharedToken};

pub struct Lexer {
    reader: CharReader,
}

impl Lexer {
    pub fn new(file: &str) -> Self {
        Self {
            reader: CharReader::new(file),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.reader.skip_whitespace();

        match self.reader.current_char() {
            Some(';') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Semicolon)
            }
            Some('{') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Lbrace)
            }
            Some('}') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Rbrace)
            }
            Some('(') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Lparentheses)
            }
            Some(')') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Rparentheses)
            }
            Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') => {
                let identifier = self.reader.read_identifier();

                match identifier.as_str() {
                    "package" => SharedToken::Extended(JavaTokenExtension::Package),
                    "public" => SharedToken::Extended(JavaTokenExtension::Public),
                    "class" => SharedToken::Extended(JavaTokenExtension::Class),
                    _ => SharedToken::Common(CommonToken::Identifier(identifier)),
                }
            }
            Some(c) => panic!("Caractère invalide: {}", c),
            None => SharedToken::Common(CommonToken::EOF),
        }
    }
}
