use std::io;

use clap::{command, Arg};
use mylang_bytecode::Inst;
use mylang_cli_ext::{read, reader_from_stdin_or_file, FileFormat, FILE_FORMAT_PARSER};

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .value_parser((*FILE_FORMAT_PARSER).clone())
                .default_value("binary")
                .help("Format of input bytecode"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read input bytecode from stdin"),
        )
        .arg(
            Arg::new("input")
                .required(false)
                .help("Input bytecode file"),
        )
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.contains_id("stdin");
    let input = matches.get_one::<String>("input").cloned();

    let stdin = io::stdin();
    let byte_code = reader_from_stdin_or_file(&stdin, use_stdin, input)?;

    let insts: Vec<Inst> = read(byte_code, &input_format)?;

    mylang_vm::execute(&insts)?;
    Ok(())
}
