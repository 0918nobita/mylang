use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::anyhow;
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

    let (tokens, errors): (Vec<_>, Vec<_>) = lexer::lex(&mut src).partition(Result::is_ok);

    let tokens = tokens.into_iter().map(Result::unwrap).collect::<Vec<_>>();

    let errors = errors
        .into_iter()
        .map(Result::unwrap_err)
        .collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&tokens)?;
    writeln!(dest, "{}", json)?;

    if errors.is_empty() {
        Ok(())
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} tokenization errors occurred", errors.len()))
    }
}
