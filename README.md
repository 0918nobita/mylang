# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

## AST を解釈実行する場合

```bash
cd examples
# Generate hello.tok.json
cargo run --bin tokenizer -- hello.mylang
# Generate hello.ast.json
cargo run --bin parser -- hello.tok.json
cargo run --bin ast_interp -- hello.ast.json
```

## AST をバイトコードに変換してから VM で実行する場合

```bash
cd examples
# Generate hello.tok.json
cargo run --bin tokenizer -- hello.mylang
# Generate hello.ast.json
cargo run --bin parser -- hello.tok.json
# Generate hello.bytecode
cargo run --bin ast_to_bytecode -- hello.ast.json
cargo run --bin bytecode_interp -- hello.bytecode
```
