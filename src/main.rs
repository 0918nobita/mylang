#[allow(dead_code)]
#[derive(Debug, Default)]
struct Pos {
    line: u32,
    column: u32,
}

#[derive(Debug)]
enum Expr {
    I32Lit(Pos, i32),
    Add(Pos, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
enum Stmt {
    Print(Pos, Expr),
}

fn main() {
    let program = Stmt::Print(
        Pos::default(),
        Expr::Add(
            Pos::default(),
            Box::new(Expr::I32Lit(Pos::default(), 3)),
            Box::new(Expr::I32Lit(Pos::default(), 4)),
        ),
    );
    println!("{:?}", program);
}
