#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommonToken {
    Identifier(String),

    Dot,
    Colon,
    Equals,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Semicolon,
    Lparentheses,
    Rparentheses,

    EOF,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<Extension> {
    Common(CommonToken),
    Extended(Extension),
}
