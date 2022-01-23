use mylang::{Expr, Range, Stmt};

extern crate mylang;

fn main() {
    let program = Stmt::Print(
        Range::default(),
        Expr::Add(
            Range::default(),
            Box::new(Expr::I32Lit(Range::default(), 3)),
            Box::new(Expr::I32Lit(Range::default(), 4)),
        ),
    );
    println!("{:?}", program);
}
