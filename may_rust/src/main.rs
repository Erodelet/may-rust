use std::fs::read_to_string;
use std::error::Error;
use crate::parser::Parser;

pub mod parser;

fn main()  -> Result<(), Box<dyn Error>> {
    let path = "./src/speadl_files/Traceur.speadl";

    let source = read_to_string(path)?;

    let mut parser = Parser::new(&source);

    parser.next_token();
    parser.namespace();

    println!("Syntaxe valide");

    Ok(())
}