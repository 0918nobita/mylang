extern crate ast_to_bytecode;

use std::fs;

use anyhow::Context;
use ast::{expr::Expr, stmt::Stmt};
use ast_to_bytecode::ast_to_bytecode;
use bytecode::Inst;

fn main() -> anyhow::Result<()> {
    let lhs = Expr::I32Lit(None, 3);
    let rhs = Expr::I32Lit(None, 4);
    let add_expr = Expr::Add(None, Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::Print(None, add_expr);

    let insts = ast_to_bytecode(&[stmt]);
    let encoded = bincode::serialize(&insts)?;
    fs::write("out.bin", encoded).context("Failed to output bytecode")?;

    let decoded: Vec<Inst> = bincode::deserialize(&fs::read("out.bin")?)?;
    println!("{:?}", decoded);

    Ok(())
}
