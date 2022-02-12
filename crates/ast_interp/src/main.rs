use std::{fs::File, io::BufReader};

use ast::Stmt;
use clap::Parser;

use ast_interp::execute;

#[derive(Parser)]
struct Opts {
    input: String,
}

fn main() -> anyhow::Result<()> {
    let Opts { input } = Opts::parse();

    let file = File::open(&input)?;
    let reader = BufReader::new(file);

    let stmts: Vec<Stmt> = serde_json::from_reader(reader)?;

    execute(&stmts)?;

    Ok(())
}
