use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::bail;
use clap::{app_from_crate, Arg};
use mylang_bytecode::Inst;
use mylang_cli_ext::FileFormat;

fn main() -> anyhow::Result<()> {
    let matches = app_from_crate!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .takes_value(true)
                .possible_values(FileFormat::possible_values())
                .default_value("binary")
                .help("Format of input bytecode"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .takes_value(false)
                .help("Read input bytecode from stdin"),
        )
        .arg(
            Arg::new("input")
                .required(false)
                .help("Input bytecode file"),
        )
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.is_present("stdin");
    let input = matches.value_of("input");

    let stdin = io::stdin();
    let byte_code: Box<dyn BufRead> = match (use_stdin, input) {
        (true, Some(_)) => bail!("Cannot specify both --stdin and [input]"),

        (true, None) => Box::new(stdin.lock()),

        (false, Some(path)) => Box::new(BufReader::new(File::open(path)?)),

        (false, None) => bail!("No input specified. You can specify either --stdin or [input]"),
    };

    let insts: Vec<Inst> = match input_format {
        FileFormat::Json => serde_json::from_reader(byte_code)?,
        FileFormat::Binary => bincode::deserialize_from(byte_code)?,
    };

    mylang_vm::execute(&insts)?;
    Ok(())
}
