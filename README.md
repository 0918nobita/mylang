# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

## 実行方法

### AST を解釈実行する場合

```bash
cd examples
cargo run --bin mylang_lexer -- -o hello.tok.json hello.mylang
cargo run --bin mylang_parser -- hello.tok.json hello.ast.json
cargo run --bin mylang_ast_interp -- hello.ast.json
```

### AST をバイトコードに変換してから VM で実行する場合

```bash
cd examples
./hello
```

## ワークスペースについて

- [仮想マシン](./crates/vm/README.md)
- [CLI 実装支援ライブラリ](./crates/cli_ext/README.md)
- [言語サーバ](./crates/lsp_server/README.md)
