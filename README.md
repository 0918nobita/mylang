# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

## ワークスペースについて

### バイナリ

- [レキサ](./crates/lexer): ソースコードをトークン列に変換する
- [パーサ](./crates/parser): トークン列を AST に変換する
- [抽象構文木インタプリタ](./crates/ast_interp): AST を解釈実行する
- [バイトコードコンパイラ](./crates/bytecode_compiler): AST をバイトコードに変換する
- [仮想マシン](./crates/vm): バイトコードを実行する
- [言語サーバ](./crates/lsp_server)

### ライブラリ

- [トークン](./crates/token)
- [抽象構文木](./crates/ast)
- [バイトコード](./crates/bytecode)
- [CLI 実装支援ライブラリ](./crates/cli_ext)

## サンプルコードの実行方法

### AST を解釈実行する場合

```bash
cd examples
cargo run --bin mylang_lexer -- -o hello.tok.json hello.mylang
cargo run --bin mylang_parser -- -o hello.ast.json hello.tok.json
cargo run --bin mylang_ast_interp -- hello.ast.json
```

### AST をバイトコードに変換してから VM で実行する場合

```bash
cd examples
./hello
```
