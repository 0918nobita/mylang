#![feature(path_file_prefix)]

extern crate parser;

use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

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
            "{}.ast.json",
            output.file_prefix().unwrap().to_str().unwrap()
        )
    });

    let src = File::open(&input)?;
    let mut src = BufReader::new(src);
    let tokens = parser::tokenize(&mut src);

    let ast = parser::parse(&tokens);

    let json = serde_json::to_string_pretty(&ast)?;
    let mut file = File::create(output)?;
    writeln!(file, "{}", json)?;

    Ok(())
}
