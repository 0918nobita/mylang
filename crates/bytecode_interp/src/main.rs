use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use bytecode::Inst;
use clap::Parser;

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

    bytecode_interp::execute(insts.into_iter())
}
