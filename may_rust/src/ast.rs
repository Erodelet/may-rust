#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    SEQ(Vec<Ast>),

    Import {
        path: Vec<String>,
    },

    Namespace {
        path: Vec<String>,
        body: Box<Ast>,
    },

    Component {
        name: String,
        specializes: Option<String>,
        generic: Option<String>,
        body: Box<Ast>,
    },

    Requires {
        name: String,
        type_name: String,
    },

    Provides {
        name: String,
        type_name: String,
        source: Option<Vec<String>>,
    },

    Part {
        name: String,
        type_name: String,
        generic: Option<String>,
        body: Box<Ast>,
    },

    Bind {
        name: String,
        target: Vec<String>,
    },
}
