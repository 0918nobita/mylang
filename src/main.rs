extern crate mylang;

use mylang::{
    exec::execute,
    expr::{Expr, ExprAst},
    stmt::{Stmt, StmtAst},
};

fn main() {
    let lhs = Expr::new(ExprAst::I32Lit(3));
    let rhs = Expr::new(ExprAst::I32Lit(4));
    let add_expr = Expr::new(ExprAst::Add(Box::new(lhs), Box::new(rhs)));
    let stmt = Stmt::new(StmtAst::Print(add_expr));
    println!("{:?}", stmt);

    execute(&[stmt]);
}
