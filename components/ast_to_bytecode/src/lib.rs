use ast::{expr::Expr, stmt::Stmt};
use bytecode::Inst;

fn expr_to_bytecode(expr: &Expr) -> Vec<Inst> {
    match expr {
        Expr::I32Lit(_, i) => vec![Inst::I32Const(*i)],
        Expr::Add(_, lhs, rhs) => {
            let mut insts = expr_to_bytecode(lhs);
            insts.extend(expr_to_bytecode(rhs));
            insts.push(Inst::I32Add);
            insts
        }
        Expr::StrLit(_, s) => vec![Inst::StrConst(s.clone())],
    }
}

fn stmt_to_bytecode(stmt: &Stmt) -> Vec<Inst> {
    match stmt {
        Stmt::Print(_, ref expr) => {
            let mut insts = expr_to_bytecode(expr);
            insts.push(Inst::Print);
            insts
        }
    }
}

pub fn ast_to_bytecode(ast: &[Stmt]) -> Vec<Inst> {
    let mut insts = vec![];
    for stmt in ast {
        insts.extend(stmt_to_bytecode(stmt));
    }
    insts
}
