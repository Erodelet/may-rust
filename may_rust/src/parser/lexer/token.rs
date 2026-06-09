#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),

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
