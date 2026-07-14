use super::token::{SpeadlTokenExtension, Token};
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

    pub fn from(lexer: &Lexer) -> Self {
        Self {
            reader: lexer.reader.reset(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.reader.skip_whitespace();

        match self.reader.current_char() {
            Some('.') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Dot)
            }
            Some(':') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Colon)
            }
            Some('=') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Equals)
            }
            Some('{') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Lbrace)
            }
            Some('}') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Rbrace)
            }
            Some('[') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Lbracket)
            }
            Some(']') => {
                self.reader.next_char();
                SharedToken::Common(CommonToken::Rbracket)
            }

            Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') => {
                let ident = self.reader.read_identifier();

                match ident.as_str() {
                    "import" => SharedToken::Extended(SpeadlTokenExtension::Import),
                    "namespace" => SharedToken::Extended(SpeadlTokenExtension::Namespace),
                    "component" => SharedToken::Extended(SpeadlTokenExtension::Component),
                    "specializes" => SharedToken::Extended(SpeadlTokenExtension::Specializes),
                    "provides" => SharedToken::Extended(SpeadlTokenExtension::Provides),
                    "requires" => SharedToken::Extended(SpeadlTokenExtension::Requires),
                    "part" => SharedToken::Extended(SpeadlTokenExtension::Part),
                    "bind" => SharedToken::Extended(SpeadlTokenExtension::Bind),
                    "to" => SharedToken::Extended(SpeadlTokenExtension::To),
                    _ => SharedToken::Common(CommonToken::Identifier(ident)),
                }
            }

            Some(c) => {
                panic!("Caractère invalide: {}", c);
            }

            None => SharedToken::Common(CommonToken::EOF),
        }
    }
}
