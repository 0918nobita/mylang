use std::fs;

use anyhow::{bail, Context};
use bytecode::Inst;
use clap::Parser;
use entity::{Entity, I32Entity, StrEntity};

#[derive(Parser)]
struct Opts {
    #[clap(short = 'i', default_value = "out.bin")]
    input: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let byte_code = fs::read(opts.input)?;
    let insts: Vec<Inst> = bincode::deserialize(&byte_code)?;

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

            Inst::Print => {
                let ent = stack.pop().context("Failed to get entity")?;

                match ent {
                    Entity::I32(i) => println!("{:?}", i),
                    Entity::Str(s) => println!("{:?}", s),
                }
            }
        }
    }

    Ok(())
}
