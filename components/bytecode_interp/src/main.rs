use std::fs;

use bytecode::Inst;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short = 'i', default_value = "out.bin")]
    input: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let insts: Vec<Inst> = bincode::deserialize(&fs::read(opts.input)?)?;
    println!("{:?}", insts);
    Ok(())
}
