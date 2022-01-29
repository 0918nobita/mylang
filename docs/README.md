# 実装に関するドキュメント

※ 掲載しているコードは擬似コードであり、そのままビルド・実行できるものではありません。

## Lexer

```rust
fn lex<T: BufRead>(src: T) -> impl Iterator<Item = LexResult>
```

- エラー復帰機能がほしい。
- 先読みを可能にするために、キャッシュ機能付きのイテレータを実装する必要がある。
- ある「状態」を前提として、その「文脈」をもとに順次受け取った文字を字句解析して結果を返すものを `LexRead` と呼ぶ。
    - この機能群を `LexRead` trait として表現する。
- 複数の `LexRead` を管理し、それぞれに適切なタイミングでトークンを生成させ、イテレータとして順次流すものを `LexSystem` と呼ぶ。

### `LexRead` trait

```rust
fn ask(pos: Pos, c: char) -> Option<Self>
```

新しく読み込んだ１文字 `c` を、この `LexRead` が最初の文字として受理できるならば `LexRead` インスタンスを生成して返す。受理できないなら `None` を返す。

```rust
fn advance(&mut self, c: char) -> LexMsg
```

新しく読み込んだ１文字 `c` に対して、それを受理するかどうか判断してメッセージを返す。

### `LexMsg` enum

```rust
enum LexMsg {
    // まだ文字の読み込みを続けられる
    Continued,

    // 受理できない文字であるが、
    // 前回までに読み込んだ文字だけでトークンを生成できるため、
    // それを生成して LexRead インスタンスを正常終了する
    Interrupted(Token),

    // 受理できない文字であり、
    // 前回までに読み込んだ文字だけはトークンを生成できないため、
    // LexRead インスタンスを異常終了する
    Failed,
}
```

### `LexSystem` struct

まだどんな使い方をするかほとんど決めていない。

```rust
struct LexSystem<I>
where 
    I: Iterator<Item = (Pos, char)>
{
    chars: I,
    readers: Vec<Box<dyn LexRead>>,
}
```

## Parser

まだ手つかず
