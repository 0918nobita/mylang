use std::io;

use anyhow::anyhow;
use clap::{command, Arg};
use itertools::Itertools;
use mylang_cli_ext::{
    read, reader_from_stdin_or_file, write, writer_to_stdout_or_file, FileFormat,
    FILE_FORMAT_PARSER,
};
use mylang_token::Token;

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .value_parser((*FILE_FORMAT_PARSER).clone())
                .default_value("json")
                .help("Format of input tokens"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read tokens from stdin"),
        )
        .arg(
            Arg::new("output_format")
                .long("output_format")
                .visible_alias("of")
                .value_parser((*FILE_FORMAT_PARSER).clone())
                .default_value("json")
                .help("Format of output AST"),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .help("Write AST to stdout"),
        )
        .arg(Arg::new("input").required(false).help("Input tokens file"))
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output AST file"),
        )
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.contains_id("stdin");
    let input = matches.get_one::<String>("input").cloned();

    let output_format = FileFormat::value_of(&matches, "output_format")?;
    let use_stdout = matches.contains_id("stdout");
    let output = matches.get_one::<String>("output").cloned();

    let stdin = io::stdin();
    let src = reader_from_stdin_or_file(&stdin, use_stdin, input)?;

    let stdout = io::stdout();
    let dest = writer_to_stdout_or_file(&stdout, use_stdout, output)?;

    let tokens: Vec<Token> = read(src, &input_format)?;

    let (stmts, errors): (Vec<_>, Vec<_>) = mylang_parser::parse(tokens.into_iter())
        .into_iter()
        .partition_result();

    if errors.is_empty() {
        write(dest, &output_format, &stmts)
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} syntax errors occurred", errors.len()))
    }
}
