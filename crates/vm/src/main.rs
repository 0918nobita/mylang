use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Parser, ArgEnum};
use mylang_bytecode::Inst;

#[derive(Parser)]
struct Opts {
    input: Option<String>,

    #[clap(long = "input_format", arg_enum, default_value = "binary")]
    input_format: InputFormat,
}

#[derive(Clone, ArgEnum)]
enum InputFormat {
    Json,
    Binary,
}

fn main() -> anyhow::Result<()> {
    let Opts {
        input,
        input_format,
    } = Opts::parse();

    let stdin = io::stdin();
    let byte_code: Box<dyn BufRead> = if let Some(input) = input {
        let file = File::open(&input)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let insts: Vec<Inst> = match input_format {
        InputFormat::Json => serde_json::from_reader(byte_code)?,
        InputFormat::Binary => bincode::deserialize_from(byte_code)?,
    };

    mylang_vm::execute(&insts)?;
    Ok(())
}
