use anyhow::bail;
use ast::{expr::Expr, stmt::Stmt};
use entity::{Entity, I32Entity, StrEntity};

fn eval(expr: &Expr) -> anyhow::Result<Entity> {
    match expr {
        Expr::I32Lit(_, i) => Ok(Entity::I32(I32Entity::new(*i))),

        Expr::StrLit(_, s) => Ok(Entity::Str(StrEntity::new(s.clone()))),

        Expr::Add(_, ref lhs, ref rhs) => {
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

#[test]
fn eval_test() {
    assert!(matches!(
        eval(&Expr::I32Lit(None, 2)),
        Ok(Entity::I32(ent)) if ent == I32Entity::new(2)
    ));

    assert!(matches!(
        eval(&Expr::Add(
            None,
            Box::new(Expr::I32Lit(None, 2)),
            Box::new(Expr::I32Lit(None, 3))
        )),
        Ok(Entity::I32(ent)) if ent == I32Entity::new(5)
    ));

    assert!(matches!(
        eval(&Expr::StrLit(None, "foo".to_owned())),
        Ok(Entity::Str(ent)) if ent == StrEntity::new("foo".to_owned())
    ));

    assert!(matches!(
        eval(
            &Expr::Add(
                None,
                Box::new(Expr::I32Lit(None, 3)),
                Box::new(Expr::StrLit(None, "bar".to_owned()))
            )
        ),
        Err(e) if e.to_string() == "Type mismatch"
    ));

    assert!(matches!(
        eval(
            &Expr::Add(
                None,
                Box::new(Expr::StrLit(None, "foo".to_owned())),
                Box::new(Expr::I32Lit(None, 4))
            )
        ),
        Err(e) if e.to_string() == "Type mismatch"
    ));

    assert!(matches!(
        eval(
            &Expr::Add(
                None,
                Box::new(Expr::StrLit(None, "foo".to_owned())),
                Box::new(Expr::StrLit(None, "bar".to_owned()))
            )
        ),
        Err(e) if e.to_string() == "Type mismatch"
    ));
}

pub fn execute(stmts: &[Stmt]) -> anyhow::Result<()> {
    for stmt in stmts {
        match stmt {
            Stmt::Print(_, ref expr) => {
                let val = eval(expr)?;
                println!("{:?}", val);
            }
        }
    }

    Ok(())
}
