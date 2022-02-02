# mylang

[![CI](https://github.com/0918nobita/mylang/actions/workflows/check.yml/badge.svg)](https://github.com/0918nobita/mylang/actions/workflows/check.yml)

趣味で少しずつ作っている自作プログラミング言語処理系です。

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

## VS Code 拡張機能について

シンタックスハイライト、字句/構文解析エラーの表示機能を提供することを目指しています。

Fable (F#) を用いて開発しているため、ビルドには .NET SDK が必要です。

### 依存ツール・パッケージのインストール

```bash
dotnet tool restore
dotnet paket restore
cd language-server
pnpm i
```

### ビルド方法

```bash
cd language-server
pnpm build
```

### 起動方法

VS Code でこのリポジトリのルートディレクトリを開き、F5 または「実行 → デバッグを開始」で起動できます。
