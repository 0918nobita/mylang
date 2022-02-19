use std::io;

use clap::Parser;
use mylang_cli_ext::{CommandFromParserExt, MatchesFromParser, WithOutputExt};

#[derive(Parser)]
struct Cli {
    input: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cmd = Cli::to_command().with_output();
    let matches = cmd.get_matches();
    let Cli { input } = matches.parse();
    println!("input: {:?}", input);
    let stdout = io::stdout();
    let _dest = matches.output(&stdout)?;
    Ok(())
}
