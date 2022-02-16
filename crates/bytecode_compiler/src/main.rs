use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::bail;
use clap::{app_from_crate, Arg};
use mylang_ast::Stmt;
use mylang_cli_ext::{FileFormat, FILE_FORMAT_POSSIBLE_VALUES};

use mylang_bytecode_compiler::ast_to_bytecode;

fn main() -> anyhow::Result<()> {
    let matches = app_from_crate!()
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
        .arg(
            Arg::new("output_format")
                .long("output_format")
                .visible_alias("of")
                .possible_values((*FILE_FORMAT_POSSIBLE_VALUES).clone())
                .default_value("binary")
                .help("Format of output bytecode"),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .takes_value(false)
                .help("Write bytecode to stdout"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .takes_value(true)
                .help("Output bytecode file"),
        )
        .arg(Arg::new("input").required(false).help("Input AST file"))
        .get_matches();

    let input_format = FileFormat::value_of(&matches, "input_format")?;
    let use_stdin = matches.is_present("stdin");
    let input = matches.value_of("input");

    let output_format = FileFormat::value_of(&matches, "output_format")?;
    let use_stdout = matches.is_present("stdout");
    let output = matches.value_of("output");

    let stdin = io::stdin();
    let src: Box<dyn BufRead> = match (use_stdin, input) {
        (true, Some(_)) => bail!("Cannot specify both --stdin and [input]"),

        (true, None) => Box::new(stdin.lock()),

        (false, Some(path)) => Box::new(BufReader::new(File::open(path)?)),

        (false, None) => bail!("No input specified. You can specify either --stdin or [input]"),
    };

    let stdout = io::stdout();
    let mut dest: Box<dyn Write> = match (use_stdout, output) {
        (true, Some(_)) => bail!("Cannot specify both --stdout and --output"),

        (true, None) => Box::new(stdout.lock()),

        (false, Some(path)) => Box::new(BufWriter::new(File::create(path)?)),

        (false, None) => {
            bail!("No output specified. You can specify either --stdout or --output")
        }
    };

    let stmts: Vec<Stmt> = match input_format {
        FileFormat::Json => serde_json::from_reader(src)?,
        FileFormat::Binary => bincode::deserialize_from(src)?,
    };

    let insts = ast_to_bytecode(&stmts);

    match output_format {
        FileFormat::Json => serde_json::to_writer_pretty(&mut dest, &insts)?,
        FileFormat::Binary => bincode::serialize_into(&mut dest, &insts)?,
    }

    Ok(())
}
