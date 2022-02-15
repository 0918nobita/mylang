use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use mylang_bytecode::Inst;

#[derive(Parser)]
struct Opts {
    input: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Opts { input } = Opts::parse();

    let stdin = io::stdin();
    let byte_code: Box<dyn BufRead> = if let Some(input) = input {
        let file = File::open(&input)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let insts: Vec<Inst> = bincode::deserialize_from(byte_code)?;

    mylang_vm::execute(&insts)?;
    Ok(())
}
