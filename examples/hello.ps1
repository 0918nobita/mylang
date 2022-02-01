cargo run --bin lexer -- hello.mylang -o hello.tok.json
cargo run --bin parser -- hello.tok.json -o hello.ast.json
cargo run --bin ast_to_bytecode -- hello.ast.json -o hello.bytecode
cargo run --bin bytecode_interp -- hello.bytecode
