//! 抽象構文木→バイトコード変換器

use mylang_ast::{Expr, Stmt};
use mylang_bytecode::Inst;

fn expr_to_bytecode(expr: &Expr) -> Vec<Inst> {
    match expr {
        Expr::I32Lit(_, i) => vec![Inst::I32Const(*i)],
        Expr::Add(lhs, rhs) => {
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
        Stmt::PrintI32(_, ref expr) => {
            let mut insts = expr_to_bytecode(expr);
            insts.push(Inst::PrintI32);
            insts
        }
        Stmt::PrintStr(_, ref expr) => {
            let mut insts = expr_to_bytecode(expr);
            insts.push(Inst::PrintStr);
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

#[cfg(test)]
mod tests {
    use mylang_ast::{Expr, Stmt};
    use mylang_token::{range, Range};

    use crate::{expr_to_bytecode, stmt_to_bytecode};

    #[test]
    fn test_i32_lit_to_bytecode() {
        insta::assert_debug_snapshot!(expr_to_bytecode(&Expr::I32Lit(Range::default(), 3)));
    }

    #[test]
    fn test_str_lit_to_bytecode() {
        let bytecode = expr_to_bytecode(&Expr::StrLit(range!(0;0,0;4), "Hello".to_owned()));
        insta::assert_debug_snapshot!(bytecode);
    }

    #[test]
    fn test_addition_to_bytecode() {
        let addition = Expr::Add(
            Box::new(Expr::I32Lit(range!(0;0), 3)),
            Box::new(Expr::I32Lit(range!(0;4,0;4), 4)),
        );
        let bytecode = expr_to_bytecode(&addition);
        insta::assert_debug_snapshot!(bytecode);
    }

    #[test]
    fn test_print_i32_to_bytecode() {
        let print_i32 = Stmt::PrintI32(range!(0;0,0;8), Expr::I32Lit(range!(0;10), 3));
        let bytecode = stmt_to_bytecode(&print_i32);
        insta::assert_debug_snapshot!(bytecode);
    }

    #[test]
    fn test_print_str_to_bytecode() {
        let print_str = Stmt::PrintStr(
            range!(0;0,0;8),
            Expr::StrLit(range!(0;10,0;23), "Hello, world!".to_owned()),
        );
        let bytecode = stmt_to_bytecode(&print_str);
        insta::assert_debug_snapshot!(bytecode);
    }
}
