use crate::gen_python::GenPython;
use crate::parser::Parser;
use std::error::Error;
use std::fs::read_to_string;

pub mod ast;
pub mod gen_python;
pub mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../examples/speadl/Simple.speadl";

    let source = read_to_string(path)?;

    let mut parser = Parser::new(&source);

    parser.next_token();
    let ast = parser.namespace();

    println!("Syntaxe valide");
    println!("{:#?}", ast);

    let mut gen_p = GenPython::new(ast);
    gen_p.generate();

    Ok(())
}
