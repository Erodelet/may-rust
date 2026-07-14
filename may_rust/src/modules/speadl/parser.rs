use super::ast::{Ast, ProvidedServiceImplementation, ServiceReference, Specializes};
use super::lexer::Lexer;
use super::token::{SpeadlTokenExtension, Token};
use crate::modules::common::token::CommonToken;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct Parser {
    pub lexer: Lexer,
    pub token: Token,
    source_dir: Option<PathBuf>,
    imports: Vec<Vec<String>>,
}

impl Parser {
    pub fn new(file: &str) -> Self {
        Self {
            lexer: Lexer::new(file),
            token: Token::Common(CommonToken::EOF),
            source_dir: None,
            imports: Vec::new(),
        }
    }

    pub fn new_with_path(file: &str, path: &Path) -> Self {
        let mut parser = Self::new(file);
        parser.source_dir = path.parent().map(Path::to_path_buf);
        parser
    }

    pub fn next_token(&mut self) {
        self.token = self.lexer.next_token()
    }

    fn accept(&mut self, t: &Token) -> bool {
        if t == &self.token {
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

    fn path(&mut self) -> Vec<String> {
        let mut path = vec![self.ident()];

        while self.accept(&Token::Common(CommonToken::Dot)) {
            path.push(self.ident());
        }

        path
    }

    fn generic(&mut self) -> Option<String> {
        if self.accept(&Token::Common(CommonToken::Lbracket)) {
            let generic = self.ident();
            self.expect(
                Token::Common(CommonToken::Rbracket),
                "after generic parameter name",
            );
            Some(generic)
        } else {
            None
        }
    }

    fn part(&mut self) -> Vec<Ast> {
        let mut parts = Vec::new();

        while self.accept(&Token::Extended(SpeadlTokenExtension::Part)) {
            let name = self.ident();
            self.expect(Token::Common(CommonToken::Colon), "after part name");
            let type_name = self.ident();
            let generic = self.generic();

            self.expect(Token::Common(CommonToken::Lbrace), "before part body");
            let mut binds = Vec::new();

            while self.accept(&Token::Extended(SpeadlTokenExtension::Bind)) {
                let name = self.ident();
                self.expect(Token::Extended(SpeadlTokenExtension::To), "after bind name");
                let mut target = vec![self.ident()];

                if self.accept(&Token::Common(CommonToken::Dot)) {
                    target.push(self.ident());
                }

                binds.push(Ast::Bind { name, target });
            }

            self.expect(Token::Common(CommonToken::Rbrace), "after part body");

            parts.push(Ast::Part {
                name,
                type_name,
                generic,
                body: Box::new(Ast::SEQ(binds)),
            });
        }

        parts
    }

    fn provides(&mut self) -> Vec<Ast> {
        let mut nodes = Vec::new();

        if self.accept(&Token::Extended(SpeadlTokenExtension::Provides)) {
            loop {
                let name = self.ident();
                self.expect(
                    Token::Common(CommonToken::Colon),
                    "after provided service name",
                );
                let type_name = self.ident();
                let implementation = if self.accept(&Token::Common(CommonToken::Equals)) {
                    let part_name = self.ident();
                    self.expect(
                        Token::Common(CommonToken::Dot),
                        "between delegated part and service name",
                    );
                    let service_name = self.ident();
                    ProvidedServiceImplementation::Delegated(ServiceReference {
                        part_name,
                        service_name,
                    })
                } else {
                    ProvidedServiceImplementation::Local
                };

                nodes.push(Ast::Provides {
                    name,
                    type_name,
                    implementation,
                });

                if !self.accept(&Token::Extended(SpeadlTokenExtension::Provides)) {
                    break;
                }
            }
            nodes.extend(self.part());
        } else {
            panic!(
                "Syntax error in component body: expected at least one `provides name: Type` declaration, found {:?}.",
                self.token
            )
        }

        nodes
    }

    fn requires(&mut self) -> Vec<Ast> {
        let mut nodes = Vec::new();

        while self.accept(&Token::Extended(SpeadlTokenExtension::Requires)) {
            let name = self.ident();
            self.expect(
                Token::Common(CommonToken::Colon),
                "after required service name",
            );
            let type_name = self.ident();
            nodes.push(Ast::Requires { name, type_name });
        }

        nodes.extend(self.provides());

        nodes
    }

    fn component(&mut self) -> Ast {
        self.expect(
            Token::Extended(SpeadlTokenExtension::Component),
            "before component name",
        );
        let name = self.ident();
        let specializes = if self.accept(&Token::Extended(SpeadlTokenExtension::Specializes)) {
            let parent = self.ident();
            Some(Specializes {
                parent: parent.clone(),
                parent_file: self.search_import(parent),
            })
        } else {
            None
        };
        let generic = self.generic();

        self.expect(Token::Common(CommonToken::Lbrace), "before component body");
        let body = Ast::SEQ(self.requires());
        self.expect(Token::Common(CommonToken::Rbrace), "after component body");

        Ast::Component {
            name,
            specializes,
            generic,
            body: Box::new(body),
        }
    }

    pub fn namespace(&mut self) -> Ast {
        let mut nodes = Vec::new();

        while self.accept(&Token::Extended(SpeadlTokenExtension::Import)) {
            let path = self.path();
            self.imports.push(path.clone());
            nodes.push(Ast::Import { path, ast: None });
        }

        self.expect(
            Token::Extended(SpeadlTokenExtension::Namespace),
            "before namespace path",
        );
        let path = self.path();
        self.expect(Token::Common(CommonToken::Lbrace), "before namespace body");
        let body = self.component();
        self.expect(Token::Common(CommonToken::Rbrace), "after namespace body");

        attach_specialized_parent_to_imports(&mut nodes, &body);

        nodes.push(Ast::Namespace {
            path,
            body: Box::new(body),
        });

        Ast::SEQ(nodes)
    }

    pub fn search_import(&self, import: String) -> Option<Box<Ast>> {
        let imported_path = self
            .imports
            .iter()
            .find(|path| path.last() == Some(&import))?;
        let source_path = self.resolve_import_path(imported_path)?;
        let source = read_to_string(&source_path).ok()?;
        let mut parser = Parser::new_with_path(&source, &source_path);

        parser.next_token();
        let ast = parser.namespace();

        Some(Box::new(ast))
    }

    fn resolve_import_path(&self, import: &[String]) -> Option<PathBuf> {
        let file_name = format!("{}.speadl", import.last()?);
        let source_dir = self.source_dir.as_ref()?;

        let same_dir = source_dir.join(&file_name);
        if same_dir.is_file() {
            return Some(same_dir);
        }

        for ancestor in source_dir.ancestors() {
            let path = import
                .iter()
                .fold(ancestor.to_path_buf(), |mut path, part| {
                    path.push(part);
                    path
                });
            let path = path.with_extension("speadl");

            if path.is_file() {
                return Some(path);
            }
        }

        None
    }
}

fn attach_specialized_parent_to_imports(imports: &mut [Ast], component: &Ast) {
    let Ast::Component {
        specializes: Some(specializes),
        ..
    } = component
    else {
        return;
    };

    let Some(parent_file) = &specializes.parent_file else {
        return;
    };

    for import in imports {
        if let Ast::Import { path, ast } = import
            && path.last() == Some(&specializes.parent)
        {
            *ast = Some(parent_file.clone());
            return;
        }
    }
}
