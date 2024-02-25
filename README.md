# score-server-rust

## このリポジトリの目的
- Rustを使って何か作ること
  - Web API サーバを作成する

## Web API サーバの仕様
- 選択した問題に応じて、ユーザの点数一覧について返すようなサービスを想定

## 設計
- DBはRDBMSを採用(クラウドサービスは現時点で利用していない)

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
$ docker start (CONTAINER_ID)
$ docker attach (CONTAINER_ID)
```

## DBの準備
```Shell
$ docker run --name dv -v score_server_db_data:/var/lib/postgresql/data -e POSTGRES_USER=$(USER_NAME) -e POSTGRES_PASSWORD=$(PASSWORD) -e POSTGRES_DB=$(DB_NAME) -p 5432:5432 postgres:15-bullseye
```
上記のようにPostgreSQLを起動した状態で、score-serverのコンテナ内で下記を実行
```Shell
$ apt update
$ apt install libpq-dev
# Docker上
# lockedファイルがないと、依存関係でエラーになる
$ cargo install diesel_cli --no-default-features --features postgres --locked
$ export DATABASE_URL=postgres://$(USER_NAME):$(PASSWORD)@localhost/$(DB_NAME)
$ diesel setup
$ diesel migration generate create_(db_name)
# 各テーブルごとにフォルダを切り、up.sqlファイルを作成
$ diesel migration run
```
DBを切り戻す場合は、
```Shell
diesel migration revert
```
DBの最新のマイグレーションをもう一度やり直したい場合は、
```Shell
diesel migration redo
```

## テスト
### 単体テスト
```Shell
$ cargo test
or 
$ cargo test (func_name)
```

### 結合テスト
```Shell
$ cargo test --test (integration_file_name)
```

## 参考
[RustでMVPなWebAPIサーバを開発する方法](https://zenn.dev/tetter/books/webapi-mvp-book)
