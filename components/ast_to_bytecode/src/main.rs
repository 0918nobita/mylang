extern crate ast_to_bytecode;

use std::fs;

use anyhow::Context;
use ast::{expr::Expr, stmt::Stmt};
use ast_to_bytecode::ast_to_bytecode;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short = 'o', default_value = "out.bin")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let lhs = Expr::I32Lit(None, 3);
    let rhs = Expr::I32Lit(None, 4);
    let add_expr = Expr::Add(None, Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::PrintI32(None, add_expr);

    let insts = ast_to_bytecode(&[stmt]);
    let encoded = bincode::serialize(&insts)?;
    fs::write(opts.output, encoded).context("Failed to output bytecode")
}
