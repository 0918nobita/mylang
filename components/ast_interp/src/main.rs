extern crate ast_interp;

use ast::{expr::Expr, stmt::Stmt};

use ast_interp::execute;

fn main() -> anyhow::Result<()> {
    let lhs = Expr::I32Lit(None, 3);
    let rhs = Expr::I32Lit(None, 4);
    let add_expr = Expr::Add(None, Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::PrintI32(None, add_expr);
    println!("{:?}", stmt);

    execute(&[stmt])
}
