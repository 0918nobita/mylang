#![feature(path_file_prefix)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

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

    let mut src: Box<dyn BufRead> = if let Some(input) = input {
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

    let tokenization_results = tokenizer::tokenize(&mut src);

    tokenization_results
        .iter()
        .filter_map(|r| if let Err(ref e) = r { Some(e) } else { None })
        .for_each(|e| {
            eprintln!("{}", e);
        });

    let tokens = tokenization_results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&tokens)?;
    writeln!(dest, "{}", json)?;

    Ok(())
}
