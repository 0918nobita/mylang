//! 抽象構文木インタプリタ
//!
//! スモールステップ意味論に沿って、構文木を繰り返し簡約し結果を返す。

mod entity;

use mylang_ast::{Expr, Stmt};
use thiserror::Error;

use entity::{Entity, I32Entity, RuntimeTypeInfo, StrEntity};

#[derive(Debug, Error)]
pub enum AstInterpError {
    #[error("Type mismatch (expected: {expected}, actual: {actual})")]
    TypeMismatch {
        expected: RuntimeTypeInfo,
        actual: RuntimeTypeInfo,
    },
}

pub type AstInterpResult<T> = Result<T, AstInterpError>;

fn eval_expr(expr: &Expr) -> AstInterpResult<Entity> {
    match expr {
        Expr::I32Lit(_, i) => Ok(Entity::I32(I32Entity::new(*i))),

        Expr::StrLit(_, s) => Ok(Entity::Str(StrEntity::new(s.clone()))),

        Expr::Add(lhs, rhs) => {
            let lhs = eval_expr(lhs)?;

            let lhs = match lhs {
                Entity::I32(i32_entity) => i32_entity,
                _ => {
                    return Err(AstInterpError::TypeMismatch {
                        expected: RuntimeTypeInfo::I32,
                        actual: lhs.get_type(),
                    })
                }
            };

            let rhs = eval_expr(rhs)?;

            let rhs = match rhs {
                Entity::I32(i32_entity) => i32_entity,
                _ => {
                    return Err(AstInterpError::TypeMismatch {
                        expected: RuntimeTypeInfo::I32,
                        actual: rhs.get_type(),
                    })
                }
            };

            Ok(Entity::I32(lhs.add(&rhs)))
        }
    }
}

/// 抽象構文木を解釈実行する
pub fn execute(stmts: &[Stmt]) -> AstInterpResult<()> {
    for stmt in stmts {
        match stmt {
            Stmt::PrintI32(_, expr) => {
                let val = eval_expr(expr)?;

                let val = match val {
                    Entity::I32(i32_entity) => i32_entity,
                    _ => {
                        return Err(AstInterpError::TypeMismatch {
                            expected: RuntimeTypeInfo::I32,
                            actual: val.get_type(),
                        })
                    }
                };

                println!("{}", val);
            }
            Stmt::PrintStr(_, expr) => {
                let val = eval_expr(expr)?;

                let val = match val {
                    Entity::Str(str_entity) => str_entity,
                    _ => {
                        return Err(AstInterpError::TypeMismatch {
                            expected: RuntimeTypeInfo::Str,
                            actual: val.get_type(),
                        })
                    }
                };

                println!("{}", val);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use mylang_ast::Expr;
    use mylang_token::Range;

    use super::eval_expr;

    #[test]
    fn test_i32_lit() {
        let ent = eval_expr(&Expr::I32Lit(Range::default(), 2));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_i32_lit_plus_i32_lit() {
        let ent = eval_expr(&Expr::Add(
            Box::new(Expr::I32Lit(Range::default(), 2)),
            Box::new(Expr::I32Lit(Range::default(), 3)),
        ));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_str_lit() {
        let ent = eval_expr(&Expr::StrLit(Range::default(), "foo".to_owned()));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_i32_lit_plus_str_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::I32Lit(Range::default(), 3)),
            Box::new(Expr::StrLit(Range::default(), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    fn test_str_lit_plus_i32_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
            Box::new(Expr::I32Lit(Range::default(), 4)),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    fn test_str_lit_plus_str_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
            Box::new(Expr::StrLit(Range::default(), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }
}
