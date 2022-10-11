use std::io;

use clap::{command, Arg};
use mylang_ast::Stmt;
use mylang_cli_ext::{
    read, reader_from_stdin_or_file, write, writer_to_stdout_or_file, FileFormat,
    FILE_FORMAT_PARSER,
};

use mylang_bytecode_compiler::ast_to_bytecode;

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
        .arg(
            Arg::new("output_format")
                .long("output_format")
                .visible_alias("of")
                .value_parser((*FILE_FORMAT_PARSER).clone())
                .default_value("binary")
                .help("Format of output bytecode"),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .help("Write bytecode to stdout"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output bytecode file"),
        )
        .arg(Arg::new("input").required(false).help("Input AST file"))
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

    let stmts: Vec<Stmt> = read(src, &input_format)?;

    let insts = ast_to_bytecode(&stmts);

    write(dest, &output_format, &insts)
}
