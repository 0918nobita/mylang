#![feature(path_file_prefix)]

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
            "{}.tok.json",
            output.file_prefix().unwrap().to_str().unwrap()
        )
    });

    let src = File::open(&input)?;
    let mut src = BufReader::new(src);
    let tokenize_results = tokenizer::tokenize(&mut src);

    tokenize_results
        .iter()
        .filter_map(|r| if let Err(ref e) = r { Some(e) } else { None })
        .for_each(|e| {
            eprintln!("{}", e);
        });

    let tokens = tokenize_results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&tokens)?;
    let mut file = File::create(output)?;
    writeln!(file, "{}", json)?;

    Ok(())
}
