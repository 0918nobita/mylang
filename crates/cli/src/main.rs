use std::{fmt, fs};

use mylang_cli::{Locatable, Location, Range};

#[derive(Clone)]
struct Identifier {
    pub raw: String,
    pub range: Range,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl<'a> Locatable<'a> for Identifier {
    fn locate(&'a self) -> &'a Range {
        &self.range
    }
}

#[derive(Clone)]
struct LetKeyword {
    pub range: Range,
}

impl fmt::Debug for LetKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let")
    }
}

impl<'a> Locatable<'a> for LetKeyword {
    fn locate(&'a self) -> &'a Range {
        &self.range
    }
}

#[derive(Clone)]
struct EqualOperator {
    pub range: Range,
}

impl fmt::Debug for EqualOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=")
    }
}

impl<'a> Locatable<'a> for EqualOperator {
    fn locate(&'a self) -> &'a Range {
        &self.range
    }
}

#[derive(Debug, Clone)]
struct I32Literal {
    pub raw: i32,
    pub range: Range,
}

impl<'a> Locatable<'a> for I32Literal {
    fn locate(&'a self) -> &'a Range {
        &self.range
    }
}

#[derive(Debug, Clone)]
enum Expression {
    I32Literal(I32Literal),
}

impl<'a> Locatable<'a> for Expression {
    fn locate(&'a self) -> &'a Range {
        match self {
            Expression::I32Literal(i32_lit) => &i32_lit.locate(),
        }
    }
}

#[derive(Debug, Clone)]
struct VariableDeclaration {
    pub let_keyword: LetKeyword,
    pub ident: Identifier,
    pub eq_keyword: EqualOperator,
    pub expression: Expression,
    range: Range,
}

impl VariableDeclaration {
    pub fn new(
        let_keyword: LetKeyword,
        ident: Identifier,
        eq_keyword: EqualOperator,
        expression: Expression,
    ) -> Self {
        let start = let_keyword.locate().start.clone();
        let end = expression.locate().end.clone();
        let range = Range { start, end };

        Self {
            let_keyword,
            ident,
            eq_keyword,
            expression,
            range,
        }
    }
}

impl<'a> Locatable<'a> for VariableDeclaration {
    fn locate(&'a self) -> &'a Range {
        &self.range
    }
}

enum Ast {
    VariableDeclaration(VariableDeclaration),
}

fn compile_expr(expression: Expression) -> String {
    match expression {
        Expression::I32Literal(i32_lit) => {
            format!("{}", i32_lit.raw)
        }
    }
}

fn compile(ast: Ast) -> String {
    match ast {
        Ast::VariableDeclaration(var_decl) => {
            let ident = var_decl.ident.raw;
            let expression = compile_expr(var_decl.expression);
            format!("const {} = {};\n", ident, expression)
        }
    }
}

fn main() {
    println!("Compiling source.mylang ...");

    // let source = fs::read_to_string("source.mylang").unwrap();
    // let source = source.trim_end();
    // let source = source.replace('\n', "\\n");

    // let out = format!("'use strict';\nconsole.log('{}');\n", source);

    let range = Range {
        start: Location { line: 0, column: 0 },
        end: Location { line: 0, column: 0 },
    };
    let out = compile(Ast::VariableDeclaration(VariableDeclaration::new(
        LetKeyword {
            range: range.clone(),
        },
        Identifier {
            raw: "foo".to_owned(),
            range: range.clone(),
        },
        EqualOperator {
            range: range.clone(),
        },
        Expression::I32Literal(I32Literal { raw: 42, range }),
    )));

    println!("> out.js");
    fs::write("out.js", format!("'use strict';\n\n{}", out)).unwrap();
}
