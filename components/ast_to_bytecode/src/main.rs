#![feature(path_file_prefix)]

extern crate ast_to_bytecode;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::Context;
use ast::{expr::Expr, range::Range, stmt::Stmt};
use ast_to_bytecode::ast_to_bytecode;
use clap::Parser;

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

    let _src: Box<dyn BufRead> = if let Some(input) = input {
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

    let lhs = Expr::I32Lit(Range::default(), 3);
    let rhs = Expr::I32Lit(Range::default(), 4);
    let add_expr = Expr::Add(Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::PrintI32(Range::default(), add_expr);

    let insts = ast_to_bytecode(&[stmt]);
    let encoded = bincode::serialize(&insts)?;

    dest.write_all(&encoded)
        .context("Failed to output bytecode")
}
