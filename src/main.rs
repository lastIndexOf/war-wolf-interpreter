use std::{env, fs};

use anyhow::Result;
use war_wolf_interpreter::{parser::parser::Parser, vm::VM};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("usage: rlua <filename>");
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)?;

    let prototype = Parser::parse(source.as_str());

    println!("prototype = {prototype:#?}");

    VM::new().run(prototype);

    Ok(())
}
