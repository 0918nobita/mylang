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
    use mylang_token::range;

    use super::eval_expr;

    #[test]
    /// 整数リテラルの評価結果が変化していないことを確認する
    fn i32_lit() {
        let ent = eval_expr(&Expr::I32Lit(range!(0;0), 2));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    /// 整数リテラル同士の足し算の評価結果が変化していないことを確認する
    fn i32_lit_plus_i32_lit() {
        let ent = eval_expr(&Expr::Add(
            Box::new(Expr::I32Lit(range!(0;0), 2)),
            Box::new(Expr::I32Lit(range!(0;0), 3)),
        ));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    /// 文字列リテラルの評価結果が変化していないことを確認する
    fn str_lit() {
        let ent = eval_expr(&Expr::StrLit(range!(0;0), "foo".to_owned()));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    /// 整数リテラルと文字列リテラルの足し算で発生するエラーの内容が変化していないことを確認する
    fn i32_lit_plus_str_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::I32Lit(range!(0;0), 3)),
            Box::new(Expr::StrLit(range!(0;0), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    /// 文字列リテラルと整数リテラルの足し算で発生するエラーの内容が変化していないことを確認する
    fn str_lit_plus_i32_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::StrLit(range!(0;0), "foo".to_owned())),
            Box::new(Expr::I32Lit(range!(0;0), 4)),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    /// 文字列リテラル同士の足し算で発生するエラーの内容が変化していないことを確認する
    fn str_lit_plus_str_lit() {
        let res = eval_expr(&Expr::Add(
            Box::new(Expr::StrLit(range!(0;0), "foo".to_owned())),
            Box::new(Expr::StrLit(range!(0;0), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }
}
