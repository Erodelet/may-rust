#[derive(Debug, PartialEq)]
pub enum Token {
    //Identifier(String),
    Identifier,

    Dot,
    Colon,
    Equals,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Import,
    Namespace,
    Component,
    Specializes,
    Provides,
    Requires,
    Part,
    Bind,
    To,

    EOF,
}