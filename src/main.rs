extern crate mylang;

use mylang::{exec::execute, expr::Expr, stmt::Stmt};

fn main() -> anyhow::Result<()> {
    let lhs = Expr::I32Lit(None, 3);
    let rhs = Expr::I32Lit(None, 4);
    let add_expr = Expr::Add(None, Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::Print(None, add_expr);
    println!("{:?}", stmt);

    execute(&[stmt])
}
