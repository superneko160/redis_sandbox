# Redisと連携

## 0. Rustインストール

[公式サイト](https://www.rust-lang.org/ja/tools/install)のとおりにインストール

ここではWSL上でRustとRedisの連携テストを行う

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

一度ターミナルから `exit` する必要有

## 1. Redisのインストール

### Debian・Ubuntu

```sh
sudo apt update
sudo apt install redis-server
```

```sudo apt install redis```でもOK


### ArchLinux

```sh
sudo pacman -Sy
sudo pacman -S redis
```

> [!NOTE]
> 正常にインストールできない場合、redisをフォークして作成された `valkey` へ移行された可能性が高いので、 `valkey` をインストールする
>
> `sudo pacman -S valkey`


## 2. Redis起動

```sh
systemctl start redis-server
```

or

```sh
sytemctl start valkey
```

### 正常に動作しているかチェック

```sh
redis-cli ping
```
`PONG`と返ってきたら動作している

## 3. 実行

### データ登録

```sh
cargo run set <key> <value>
cargo run set A100 Alice
```

### データ取得

```sh
cargo run get <key>
cargo run get A100
```

## 4. 使用クレート

- [redis - Rust](https://docs.rs/redis/latest/redis/)

