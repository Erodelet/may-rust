use std::fs::read_to_string;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Token {
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

//------------LEXER-------------

struct Lexer {
    file: Vec<char>,
    ind: usize,
}

impl Lexer {
    fn new(file: &str) -> Self {
        Self {
            file: file.chars().collect(),
            ind: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.file.get(self.ind).copied()
    }

    fn next_char(&mut self) {
        self.ind += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut id = String::new();

        while let Some(c) = self.current_char() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    id.push(c);
                    self.next_char();
                }
                _ => break,
            }
        }

        id
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            Some('.') => {
                self.next_char();
                Token::Dot
            }
            Some(':') => {
                self.next_char();
                Token::Colon
            }
            Some('=') => {
                self.next_char();
                Token::Equals
            }
            Some('{') => {
                self.next_char();
                Token::Lbrace
            }
            Some('}') => {
                self.next_char();
                Token::Rbrace
            }
            Some('[') => {
                self.next_char();
                Token::Lbracket
            }
            Some(']') => {
                self.next_char();
                Token::Rbracket
            }

            Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') => {
                let ident = self.read_identifier();

                match ident.as_str() {
                    "import" => Token::Import,
                    "namespace" => Token::Namespace,
                    "component" => Token::Component,
                    "specializes" => Token::Specializes,
                    "provides" => Token::Provides,
                    "requires" => Token::Requires,
                    "part" => Token::Part,
                    "bind" => Token::Bind,
                    "to" => Token::To,
                    //_ => Token::Identifier(ident),
                    _ => Token::Identifier,
                }
            }

            Some(c) => {
                panic!("Caractère invalide: {}", c);
            }

            None => Token::EOF,
        }
    }
}

//------------PARSER------------

struct Parser {
    lexer : Lexer,
    token : Token,
}

impl Parser {
    fn new(file: &str) -> Self {
        Self {
            lexer: Lexer::new(file),
            token: Token::EOF,
        }
    }

    fn next_token(&mut self) {
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

    fn namespace(&mut self){
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

fn main()  -> Result<(), Box<dyn Error>> {
    let path = "./src/speadl_files/Traceur.speadl";

    let source = read_to_string(path)?;

    let mut parser = Parser::new(&source);

    parser.next_token();
    parser.namespace();

    println!("Syntaxe valide");

    Ok(())
}