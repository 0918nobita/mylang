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
    use token::Range;

    use super::eval;

    #[test]
    fn test_i32_lit() {
        let ent = eval(&Expr::I32Lit(Range::default(), 2));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_i32_lit_plus_i32_lit() {
        let ent = eval(&Expr::Add(
            Box::new(Expr::I32Lit(Range::default(), 2)),
            Box::new(Expr::I32Lit(Range::default(), 3)),
        ));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_str_lit() {
        let ent = eval(&Expr::StrLit(Range::default(), "foo".to_owned()));
        insta::assert_debug_snapshot!(ent);
    }

    #[test]
    fn test_i32_lit_plus_str_lit() {
        let res = eval(&Expr::Add(
            Box::new(Expr::I32Lit(Range::default(), 3)),
            Box::new(Expr::StrLit(Range::default(), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    fn test_str_lit_plus_i32_lit() {
        let res = eval(&Expr::Add(
            Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
            Box::new(Expr::I32Lit(Range::default(), 4)),
        ));
        insta::assert_debug_snapshot!(res);
    }

    #[test]
    fn test_str_lit_plus_str_lit() {
        let res = eval(&Expr::Add(
            Box::new(Expr::StrLit(Range::default(), "foo".to_owned())),
            Box::new(Expr::StrLit(Range::default(), "bar".to_owned())),
        ));
        insta::assert_debug_snapshot!(res);
    }
}
