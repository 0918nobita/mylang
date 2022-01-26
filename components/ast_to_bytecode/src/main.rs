#![feature(path_file_prefix)]

extern crate ast_to_bytecode;

use std::{fs, path::PathBuf};

use anyhow::Context;
use ast::{expr::Expr, range::Range, stmt::Stmt};
use ast_to_bytecode::ast_to_bytecode;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    input: String,

    #[clap(short = 'o')]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Opts { input, output } = Opts::parse();

    let output = output.unwrap_or_else(|| {
        let output = PathBuf::from(&input);
        format!(
            "{}.bytecode",
            output.file_prefix().unwrap().to_str().unwrap()
        )
    });

    let lhs = Expr::I32Lit(Range::default(), 3);
    let rhs = Expr::I32Lit(Range::default(), 4);
    let add_expr = Expr::Add(Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::PrintI32(Range::default(), add_expr);

    let insts = ast_to_bytecode(&[stmt]);
    let encoded = bincode::serialize(&insts)?;
    fs::write(output, encoded).context("Failed to output bytecode")
}
