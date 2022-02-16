use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::{anyhow, bail};
use clap::{app_from_crate, Arg};
use itertools::Itertools;
use mylang_cli_ext::{FileFormat, FILE_FORMAT_POSSIBLE_VALUES};
use mylang_token::Token;

fn main() -> anyhow::Result<()> {
    let matches = app_from_crate!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
                .default_value("json")
                .help("Format of input tokens"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .takes_value(false)
                .help("Read tokens from stdin"),
        )
        .arg(
            Arg::new("output_format")
                .long("output_format")
                .visible_alias("of")
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
                .default_value("json")
                .help("Format of output AST"),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .takes_value(false)
                .help("Write AST to stdout"),
        )
        .arg(Arg::new("input").required(false).help("Input tokens file"))
        .arg(Arg::new("output").required(false).help("Output AST file"))
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.is_present("stdin");
    let input = matches.value_of("input");

    let output_format = FileFormat::value_of(&matches, "output_format")?;
    let use_stdout = matches.is_present("stdout");
    let output = matches.value_of("output");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let src: Box<dyn BufRead> = match (use_stdin, input) {
        (true, Some(_)) => bail!("Cannot specify both --stdin and [input]"),

        (true, None) => Box::new(stdin.lock()),

        (false, Some(path)) => Box::new(BufReader::new(File::open(path)?)),

        (false, None) => bail!("No input specified. You can specify either --stdin or [input]"),
    };

    let mut dest: Box<dyn Write> = match (use_stdout, output) {
        (true, Some(_)) => bail!("Cannot specify both --stdout and [output]"),

        (true, None) => Box::new(stdout.lock()),

        (false, Some(path)) => Box::new(BufWriter::new(File::create(path)?)),

        (false, None) => bail!("No output specified. You can specify either --stdout or [output]"),
    };

    let tokens: Vec<Token> = match input_format {
        FileFormat::Json => serde_json::from_reader(src)?,
        FileFormat::Binary => bincode::deserialize_from(src)?,
    };

    let (stmts, errors): (Vec<_>, Vec<_>) = mylang_parser::parse(tokens.into_iter())
        .into_iter()
        .partition_result();

    if errors.is_empty() {
        match output_format {
            FileFormat::Json => serde_json::to_writer_pretty(&mut dest, &stmts)?,
            FileFormat::Binary => bincode::serialize_into(&mut dest, &stmts)?,
        };
        Ok(())
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} syntax errors occurred", errors.len()))
    }
}
