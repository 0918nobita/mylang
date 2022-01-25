# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

## AST を解釈実行する場合

```bash
cargo run --bin ast_interp
```

## AST をバイトコードに変換してから VM で実行する場合

```bash
cargo run --bin ast_to_bytecode
cargo run --bin bytecode_interp
```
