use std::io;

use anyhow::anyhow;
use clap::{command, Arg};
use itertools::Itertools;
use mylang_cli_ext::{
    reader_from_stdin_or_file, write, writer_to_stdout_or_file, FileFormat,
    FILE_FORMAT_POSSIBLE_VALUES,
};
use utf8_chars::BufReadCharsExt;

fn main() -> anyhow::Result<()> {
    let app = command!()
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .takes_value(false)
                .help("Read source code from stdin"),
        )
        .arg(
            Arg::new("output_format")
                .long("output_format")
                .visible_alias("of")
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
                .default_value("json")
                .help("Format of output tokens"),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .takes_value(false)
                .help("Write tokens to stdout"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .takes_value(true)
                .help("Output tokens file"),
        )
        .arg(Arg::new("input").required(false).help("Input source file"));
    let matches = app.get_matches();

    let use_stdin = matches.is_present("stdin");
    let input = matches.value_of("input");

    let output_format = FileFormat::value_of(&matches, "output_format")?;
    let use_stdout = matches.is_present("stdout");
    let output = matches.value_of("output");

    let stdin = io::stdin();
    let mut src = reader_from_stdin_or_file(&stdin, use_stdin, input)?;

    let stdout = io::stdout();
    let dest = writer_to_stdout_or_file(&stdout, use_stdout, output)?;

    let (tokens, errors): (Vec<_>, Vec<_>) =
        mylang_lexer::lex(src.chars().map(|r| r.unwrap())).partition_result();

    if errors.is_empty() {
        write(dest, &output_format, &tokens)
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} tokenization errors occurred", errors.len()))
    }
}
