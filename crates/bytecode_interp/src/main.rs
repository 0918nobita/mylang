use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::{bail, Context};
use bytecode::Inst;
use clap::Parser;
use entity::{Entity, I32Entity, StrEntity};

#[derive(Parser)]
struct Opts {
    input: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Opts { input } = Opts::parse();

    let stdin = io::stdin();
    let byte_code: Box<dyn BufRead> = if let Some(input) = input {
        let file = File::open(&input)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let insts: Vec<Inst> = bincode::deserialize_from(byte_code)?;

    let mut stack = Vec::<Entity>::new();
    for inst in insts {
        match inst {
            Inst::I32Const(i) => stack.push(Entity::I32(I32Entity::new(i))),

            Inst::I32Add => {
                let rhs = stack.pop().context("Failed to get right-hand side")?;
                let lhs = stack.pop().context("Failed to get left-hand side")?;

                if let (Entity::I32(lhs), Entity::I32(rhs)) = (lhs, rhs) {
                    stack.push(Entity::I32(lhs.add(&rhs)));
                } else {
                    bail!("Type mismatch");
                }
            }

            Inst::StrConst(s) => stack.push(Entity::Str(StrEntity::new(s.clone()))),

            Inst::PrintI32 => {
                let ent = stack.pop().context("Failed to get entity")?;

                if let Entity::I32(ent) = ent {
                    println!("{:?}", ent);
                } else {
                    bail!("Type mismatch");
                }
            }

            Inst::PrintStr => {
                let ent = stack.pop().context("Failed to get entity")?;

                if let Entity::Str(ent) = ent {
                    println!("{:?}", ent);
                } else {
                    bail!("Type mismatch");
                }
            }
        }
    }

    Ok(())
}
