use std::io;

use clap::{command, Arg};
use mylang_bytecode::Inst;
use mylang_cli_ext::{read_from_stdin_or_file, FileFormat, FILE_FORMAT_POSSIBLE_VALUES};

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .takes_value(true)
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
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
    let byte_code = read_from_stdin_or_file(&stdin, use_stdin, input)?;

    let insts: Vec<Inst> = match input_format {
        FileFormat::Json => serde_json::from_reader(byte_code)?,
        FileFormat::Binary => bincode::deserialize_from(byte_code)?,
    };

    mylang_vm::execute(&insts)?;
    Ok(())
}
