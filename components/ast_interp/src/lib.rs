use std::fmt::{Debug, Formatter, Result as FmtResult};

use anyhow::bail;

use ast::{expr::Expr, stmt::Stmt};

struct I32Entity {
    value: i32,
}

impl I32Entity {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn add(&self, rhs: &Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Debug for I32Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

struct StrEntity {
    value: String,
}

impl StrEntity {
    fn new(value: String) -> Self {
        Self { value }
    }
}

impl Debug for StrEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
enum Entity {
    I32(I32Entity),
    Str(StrEntity),
}

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
        Ok(Entity::I32(I32Entity { value: 2 }))
    ));

    assert!(matches!(
        eval(&Expr::Add(
            None,
            Box::new(Expr::I32Lit(None, 2)),
            Box::new(Expr::I32Lit(None, 3))
        )),
        Ok(Entity::I32(I32Entity { value: 5 }))
    ));

    assert!(matches!(
        eval(&Expr::StrLit(None, "foo".to_owned())),
        Ok(Entity::Str(StrEntity { value })) if value.as_str() == "foo"
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
