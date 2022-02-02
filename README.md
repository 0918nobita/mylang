# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

## 必要なもの

- rustup
- .NET 6.0 SDK

## 実行方法

### AST を解釈実行する場合

```bash
cd examples
cargo run --bin lexer -- hello.mylang -o hello.tok.json
cargo run --bin parser -- hello.tok.json -o hello.ast.json
cargo run --bin ast_interp -- hello.ast.json
```

### AST をバイトコードに変換してから VM で実行する場合

```bash
cd examples
./hello
```
