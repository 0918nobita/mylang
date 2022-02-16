cargo build && `
    & ..\target\debug\mylang_lexer -o hello.tok.json hello.mylang && `
    & ..\target\debug\mylang_parser -o hello.ast.json hello.tok.json && `
    & ..\target\debug\mylang_bytecode_compiler -o hello.ast.json hello.bytecode && `
    & ..\target\debug\mylang_vm hello.bytecode
