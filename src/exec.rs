use std::fmt::{Debug, Formatter, Result as FmtResult};

use super::{expr::Expr, stmt::Stmt};

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

#[derive(Debug)]
enum Entity {
    I32(I32Entity),
}

fn eval(expr: &Expr) -> Entity {
    match expr {
        Expr::I32Lit(_, i) => Entity::I32(I32Entity::new(*i)),
        Expr::Add(_, ref lhs, ref rhs) => {
            let Entity::I32(lhs) = eval(lhs);
            let Entity::I32(rhs) = eval(rhs);
            Entity::I32(lhs.add(&rhs))
        }
    }
}

pub fn execute(stmts: &[Stmt]) {
    for stmt in stmts {
        match stmt {
            Stmt::Print(_, ref expr) => {
                println!("{:?}", eval(expr));
            }
        }
    }
}
