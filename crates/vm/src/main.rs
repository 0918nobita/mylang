use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{app_from_crate, Arg};
use mylang_bytecode::Inst;
use mylang_cli_ext::{input_format_arg, value_of_input_format, InputFormat};

fn main() -> anyhow::Result<()> {
    let matches = app_from_crate!()
        .arg(Arg::new("input").required(false))
        .arg(input_format_arg().default_value("binary"))
        .get_matches();
    let input_format = value_of_input_format(&matches)?;
    let input = matches.value_of("input");

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
