cargo build && `
    & ..\target\debug\mylang_lexer -o hello.tok.json hello.mylang && `
    & ..\target\debug\mylang_parser hello.tok.json hello.ast.json && `
    & ..\target\debug\mylang_bytecode_compiler hello.ast.json hello.bytecode && `
    & ..\target\debug\mylang_vm hello.bytecode
