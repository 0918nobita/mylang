use anyhow::bail;
use ast::{Expr, Stmt};
use entity::{Entity, I32Entity, StrEntity};

fn eval(expr: &Expr) -> anyhow::Result<Entity> {
    match expr {
        Expr::I32Lit(_, i) => Ok(Entity::I32(I32Entity::new(*i))),

        Expr::StrLit(_, s) => Ok(Entity::Str(StrEntity::new(s.clone()))),

        Expr::Add(ref lhs, ref rhs) => {
            let lhs = eval(lhs)?;
            let rhs = eval(rhs)?;

            if let (Entity::I32(lhs), Entity::I32(rhs)) = (lhs, rhs) {
                Ok(Entity::I32(lhs.add(&rhs)))
            } else {
                bail!("Type mismatch")
            }
        }
    }
}

pub fn execute(stmts: &[Stmt]) -> anyhow::Result<()> {
    for stmt in stmts {
        match stmt {
            Stmt::PrintI32(_, ref expr) => {
                let val = eval(expr)?;
                if let Entity::I32(ent) = val {
                    println!("{:?}", ent);
                } else {
                    bail!("Type mismatch");
                }
            }
            Stmt::PrintStr(_, ref expr) => {
                let val = eval(expr)?;
                if let Entity::Str(ent) = val {
                    println!("{:?}", ent);
                } else {
                    bail!("Type mismatch");
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ast::Expr;
    use entity::{Entity, I32Entity, StrEntity};
    use token::Range;

    use super::eval;

    #[test]
    fn eval_test() {
        assert!(matches!(
            eval(&Expr::I32Lit(Range::default(), 2)),
            Ok(Entity::I32(ent)) if ent == I32Entity::new(2)
        ));

        assert!(matches!(
            eval(&Expr::Add(
                Box::new(Expr::I32Lit(Range::default(), 2)),
                Box::new(Expr::I32Lit(Range::default(), 3))
            )),
            Ok(Entity::I32(ent)) if ent == I32Entity::new(5)
        ));

        assert!(matches!(
            eval(&Expr::StrLit(Range::default(), "foo".to_owned())),
            Ok(Entity::Str(ent)) if ent == StrEntity::new("foo".to_owned())
        ));

        assert!(matches!(
            eval(
                &Expr::Add(
                    Box::new(Expr::I32Lit(Range::default(), 3)),
                    Box::new(Expr::StrLit(Range::default(), "bar".to_owned()))
                )
            ),
            Err(e) if e.to_string() == "Type mismatch"
        ));

        assert!(matches!(
            eval(
                &Expr::Add(
                    Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
                    Box::new(Expr::I32Lit(Range::default(), 4))
                )
            ),
            Err(e) if e.to_string() == "Type mismatch"
        ));

        assert!(matches!(
            eval(
                &Expr::Add(
                    Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
                    Box::new(Expr::StrLit(Range::default(), "bar".to_owned()))
                )
            ),
            Err(e) if e.to_string() == "Type mismatch"
        ));
    }
}
