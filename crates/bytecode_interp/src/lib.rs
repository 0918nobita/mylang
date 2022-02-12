//! バイトコードインタプリタ

use bytecode::Inst;
use entity::{Entity, I32Entity, StrEntity};
use thiserror::Error;

/// バイトコードインタプリタで発生するエラー
#[derive(Debug, Error)]
pub enum InterpError {
    /// スタックが空であり、値をスタックから取り出せなかった
    #[error("Stack underflow ({0})")]
    StackUnderflow(String),

    /// 取り出した値が、期待した型の値ではなかった
    #[error("Type mismatch")]
    TypeMismatch,
}

/// バイトコードの実行結果
type InterpResult<T> = Result<T, InterpError>;

/// I32Const 命令を実行する
fn i32_const(stack: &mut Vec<Entity>, immediate: i32) {
    stack.push(Entity::I32(I32Entity::new(immediate)));
}

/// I32Add 命令を実行する
fn i32_add(stack: &mut Vec<Entity>) -> InterpResult<()> {
    let lhs = stack.pop().ok_or_else(|| {
        InterpError::StackUnderflow("Failed to get left-hand side of the addition".to_owned())
    })?;
    let rhs = stack.pop().ok_or_else(|| {
        InterpError::StackUnderflow("Failed to get right-hand side of the addition".to_owned())
    })?;

    if let (Entity::I32(lhs), Entity::I32(rhs)) = (lhs, rhs) {
        stack.push(Entity::I32(lhs.add(&rhs)));
        Ok(())
    } else {
        Err(InterpError::TypeMismatch)
    }
}

/// StrConst 命令を実行する
fn str_const(stack: &mut Vec<Entity>, immediate: &str) {
    stack.push(Entity::Str(StrEntity::new(immediate.to_owned())));
}

/// PrintI32 命令を実行する
fn print_i32(stack: &mut Vec<Entity>) -> InterpResult<()> {
    let ent = stack.pop().ok_or_else(|| {
        InterpError::StackUnderflow("Failed to get the entity to output".to_owned())
    })?;

    if let Entity::I32(ent) = ent {
        println!("{}", ent);
        Ok(())
    } else {
        Err(InterpError::TypeMismatch)
    }
}

/// PrintStr 命令を実行する
fn print_str(stack: &mut Vec<Entity>) -> InterpResult<()> {
    let ent = stack
        .pop()
        .ok_or_else(|| InterpError::StackUnderflow("Failed to get entity".to_owned()))?;

    if let Entity::Str(ent) = ent {
        println!("{}", ent);
        Ok(())
    } else {
        Err(InterpError::TypeMismatch)
    }
}

/// バイトコードを解釈実行する
pub fn execute(insts: impl Iterator<Item = Inst>) -> InterpResult<()> {
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