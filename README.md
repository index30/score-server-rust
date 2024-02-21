# score-server-rust

## このリポジトリの目的
- Rustを使って何か作ること
  - Web API サーバを作成する

## Web API サーバの仕様
- 選択した試験に応じて、その試験の概要について返すようなサービスを想定

## 開発時の環境整備

### 初回時
```Shell
$ git clone git@github.com:index30/score-server-rust.git
$ cd score-server-rust
$ docker run -it --name rust-dev -v "$(pwd)":/usr/src/score-server -e HOME=/usr/src/score-server --net=host rust:1.70-slim-buster /bin/bash
$ cargo new score-server-rust
```

### スプリントを回す時
```Shell
$ cd score-server-rust
$ docker run -it --name rust-dev -v "$(pwd)":/usr/src/score-server -e HOME=/usr/src/score-server --net=host rust:1.70-slim-buster /bin/bash
```

## 参考
[RustでMVPなWebAPIサーバを開発する方法](https://zenn.dev/tetter/books/webapi-mvp-book)
