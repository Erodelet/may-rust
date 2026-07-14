use crate::modules::common::token::Token as SharedToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaTokenExtension {
    Package,
    Public,
    Class,
}

pub type Token = SharedToken<JavaTokenExtension>;
