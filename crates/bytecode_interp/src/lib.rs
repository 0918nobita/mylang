use anyhow::{bail, Context};
use bytecode::Inst;
use entity::{Entity, I32Entity, StrEntity};

fn i32_const(stack: &mut Vec<Entity>, immediate: i32) {
    stack.push(Entity::I32(I32Entity::new(immediate)));
}

fn i32_add(stack: &mut Vec<Entity>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("Failed to get left-hand side")?;
    let rhs = stack.pop().context("Failed to get right-hand side")?;

    if let (Entity::I32(lhs), Entity::I32(rhs)) = (lhs, rhs) {
        stack.push(Entity::I32(lhs.add(&rhs)));
        Ok(())
    } else {
        bail!("Type mismatch");
    }
}

fn str_const(stack: &mut Vec<Entity>, immediate: &str) {
    stack.push(Entity::Str(StrEntity::new(immediate.to_owned())));
}

fn print_i32(stack: &mut Vec<Entity>) -> anyhow::Result<()> {
    let ent = stack.pop().context("Failed to get entity")?;

    if let Entity::I32(ent) = ent {
        println!("{}", ent);
        Ok(())
    } else {
        bail!("Type mismatch");
    }
}

fn print_str(stack: &mut Vec<Entity>) -> anyhow::Result<()> {
    let ent = stack.pop().context("Failed to get entity")?;

    if let Entity::Str(ent) = ent {
        println!("{}", ent);
        Ok(())
    } else {
        bail!("Type mismatch");
    }
}

pub fn execute(insts: impl Iterator<Item = Inst>) -> anyhow::Result<()> {
    let mut stack = Vec::<Entity>::new();

    for inst in insts {
        match inst {
            Inst::I32Const(i) => i32_const(&mut stack, i),

            Inst::I32Add => i32_add(&mut stack)?,

            Inst::StrConst(s) => str_const(&mut stack, &s),

            Inst::PrintI32 => print_i32(&mut stack)?,

            Inst::PrintStr => print_str(&mut stack)?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use entity::{Entity, I32Entity, StrEntity};

    use crate::i32_add;

    #[test]
    fn test_i32_add() {
        let mut stack = vec![
            Entity::I32(I32Entity::new(3)),
            Entity::I32(I32Entity::new(4)),
        ];
        insta::assert_debug_snapshot!((i32_add(&mut stack), stack));
    }

    #[test]
    fn test_i32_add_underflow() {
        let mut stack = vec![Entity::I32(I32Entity::new(3))];
        insta::assert_debug_snapshot!((i32_add(&mut stack), stack));
    }

    #[test]
    fn test_i32_add_type_mismatch() {
        let mut stack = vec![
            Entity::Str(StrEntity::new("foo".to_owned())),
            Entity::I32(I32Entity::new(4)),
        ];
        insta::assert_debug_snapshot!((i32_add(&mut stack), stack));
    }
}
