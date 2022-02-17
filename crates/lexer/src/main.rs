use std::io;

use anyhow::anyhow;
use clap::{command, Arg};
use itertools::Itertools;
use mylang_cli_ext::{
    read_from_stdin_or_file, write_to_stdout_or_file, FileFormat, FILE_FORMAT_POSSIBLE_VALUES,
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
    let mut src = read_from_stdin_or_file(&stdin, use_stdin, input)?;

    let stdout = io::stdout();
    let mut dest = write_to_stdout_or_file(&stdout, use_stdout, output)?;

    let (tokens, errors): (Vec<_>, Vec<_>) =
        mylang_lexer::lex(src.chars().map(|r| r.unwrap())).partition_result();

    if errors.is_empty() {
        match output_format {
            FileFormat::Json => serde_json::to_writer_pretty(&mut dest, &tokens)?,
            FileFormat::Binary => bincode::serialize_into(&mut dest, &tokens)?,
        };
        Ok(())
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} tokenization errors occurred", errors.len()))
    }
}
