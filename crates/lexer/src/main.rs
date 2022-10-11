use std::io;

use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use itertools::Itertools;
use mylang_cli_ext::{reader_from_stdin_or_file, write, writer_to_stdout_or_file, FileFormat};
use utf8_chars::BufReadCharsExt;

#[derive(Parser)]
#[clap(version, author, about)]
#[clap(group(ArgGroup::new("output_group").required(true)))]
struct Cli {
    /// Input source file
    input: Option<String>,

    /// Read source code from stdin
    #[clap(long = "stdin")]
    use_stdin: bool,

    /// Format of output tokens
    #[clap(long, value_enum, default_value = "json")]
    output_format: FileFormat,

    /// Write tokens to stdout
    #[clap(long = "stdout", group = "output_group")]
    use_stdout: bool,

    /// Output tokens file
    #[clap(short, long, group = "output_group")]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Cli {
        use_stdin,
        input,
        output_format,
        use_stdout,
        output,
    } = Cli::parse();

    let stdin = io::stdin();
    let mut src = reader_from_stdin_or_file(&stdin, use_stdin, input)?;

    let stdout = io::stdout();
    let dest = writer_to_stdout_or_file(&stdout, use_stdout, output)?;

    let (tokens, errors): (Vec<_>, Vec<_>) =
        mylang_lexer::lex(src.chars().map(|r| r.unwrap())).partition_result();

    if errors.is_empty() {
        write(dest, &output_format, &tokens)
    } else {
        for err in errors.iter() {
            eprintln!("{}", err);
        }
        Err(anyhow!("{} tokenization errors occurred", errors.len()))
    }
}
