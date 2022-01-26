extern crate ast_interp;

use ast::{expr::Expr, range::Range, stmt::Stmt};

use ast_interp::execute;

fn main() -> anyhow::Result<()> {
    let lhs = Expr::I32Lit(Range::default(), 3);
    let rhs = Expr::I32Lit(Range::default(), 4);
    let add_expr = Expr::Add(Box::new(lhs), Box::new(rhs));
    let stmt = Stmt::PrintI32(Range::default(), add_expr);
    println!("{:?}", stmt);

    execute(&[stmt])
}
