use crate::modules::common::token::Token as SharedToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpeadlTokenExtension {
    Import,
    Namespace,
    Component,
    Specializes,
    Provides,
    Requires,
    Part,
    Bind,
    To,
}

pub type Token = SharedToken<SpeadlTokenExtension>;
