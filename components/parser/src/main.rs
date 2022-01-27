#![feature(path_file_prefix)]

extern crate parser;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use ast::{expr::Expr, range::Range, stmt::Stmt};
use clap::Parser;

use parser::CharsWithPosExt;

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
            "{}.ast.json",
            output.file_prefix().unwrap().to_str().unwrap()
        )
    });

    let src = fs::read_to_string(input)?;

    // let mut tokens = Vec::<Token>::new();
    // let mut acc = String::new();
    for (pos, c) in src.chars().with_pos() {
        println!("{:?} {:?}", pos, c);
    }

    let ast = vec![Stmt::PrintStr(
        Range::default(),
        Expr::StrLit(Range::default(), "Hello, world!".to_owned()),
    )];

    let json = serde_json::to_string_pretty(&ast)?;
    let mut file = File::create(output)?;
    writeln!(file, "{}", json)?;

    Ok(())
}
