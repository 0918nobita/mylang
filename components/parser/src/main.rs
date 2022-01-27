#![feature(path_file_prefix)]

extern crate parser;

use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;
use token::Token;

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

    let tokens: Vec<Token> = serde_json::from_reader(File::open(&input)?)?;

    let ast = parser::parse(&tokens);

    let json = serde_json::to_string_pretty(&ast)?;
    let mut file = File::create(output)?;
    writeln!(file, "{}", json)?;

    Ok(())
}
