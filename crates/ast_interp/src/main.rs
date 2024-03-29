use std::io;

use clap::{command, Arg};
use mylang_ast::Stmt;
use mylang_cli_ext::{read, reader_from_stdin_or_file, FileFormat, FILE_FORMAT_PARSER};

use mylang_ast_interp::execute;

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            Arg::new("input_format")
                .long("input_format")
                .visible_alias("if")
                .value_parser((*FILE_FORMAT_PARSER).clone())
                .default_value("json")
                .help("Format of input AST"),
        )
        .arg(Arg::new("stdin").long("stdin").help("Read AST from stdin"))
        .arg(Arg::new("input").required(false).help("Input AST file"))
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.contains_id("stdin");
    let input = matches.get_one::<String>("input").cloned();

    let stdin = io::stdin();
    let src = reader_from_stdin_or_file(&stdin, use_stdin, input)?;

    let stmts: Vec<Stmt> = read(src, &input_format)?;

    execute(&stmts)?;
    Ok(())
}
