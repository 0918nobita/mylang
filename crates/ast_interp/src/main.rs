use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::bail;
use clap::{command, Arg};
use mylang_ast::Stmt;
use mylang_cli_ext::{FileFormat, FILE_FORMAT_POSSIBLE_VALUES};

use mylang_ast_interp::execute;

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
                .default_value("json")
                .help("Format of input AST"),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .takes_value(false)
                .help("Read AST from stdin"),
        )
        .arg(Arg::new("input").required(false).help("Input AST file"))
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.is_present("stdin");
    let input = matches.value_of("input");

    let stdin = io::stdin();
    let src: Box<dyn BufRead> = match (use_stdin, input) {
        (true, Some(_)) => bail!("Cannot specify both --stdin and [input]"),

        (true, None) => Box::new(stdin.lock()),

        (false, Some(path)) => Box::new(BufReader::new(File::open(path)?)),

        (false, None) => bail!("No input specified. You can specify either --stdin or [input]"),
    };

    let stmts: Vec<Stmt> = match input_format {
        FileFormat::Json => serde_json::from_reader(src)?,
        FileFormat::Binary => bincode::deserialize_from(src)?,
    };

    execute(&stmts)?;
    Ok(())
}
