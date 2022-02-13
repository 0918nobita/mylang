use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::Context;
use clap::Parser;
use mylang_ast::Stmt;

use mylang_bytecode_compiler::ast_to_bytecode;

#[derive(Parser)]
struct Opts {
    input: Option<String>,

    #[clap(short = 'o')]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Opts { input, output } = Opts::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();

    let src: Box<dyn BufRead> = if let Some(input) = input {
        let file = File::open(&input)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let mut dest: Box<dyn Write> = if let Some(output) = output {
        let file = File::create(output)?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(BufWriter::new(stdout.lock()))
    };

    let stmts: Vec<Stmt> = serde_json::from_reader(src)?;

    let insts = ast_to_bytecode(&stmts);

    let encoded = bincode::serialize(&insts)?;

    dest.write_all(&encoded)
        .context("Failed to output bytecode")
}
