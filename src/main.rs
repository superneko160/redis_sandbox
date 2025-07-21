extern crate redis;
use redis::Commands;
use std::env;

// Redisと接続
fn connection_handling() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let con = client.get_connection()?;
    Ok(con)
}

// Redisにデータを挿入
fn set_to_redis(mut con: redis::Connection, key: String, value: String) -> redis::RedisResult<()> {
    let _: () = con.set(&key, &value)?;
    Ok(())
}

// Redisからデータを取得
fn get_from_redis(mut con: redis::Connection, key: String) -> redis::RedisResult<String> {
   Ok(con.get(&key)?)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage:");
        eprintln!("  cargo run add <key> <value>");
        eprintln!("  cargo run get <key>");
        return;
    }

    let command = &args[1];

    match connection_handling() {
        Ok(con) => {
            match command.as_str() {
                "add" => {
                    if args.len() != 4 {
                        eprintln!("usage: cargo run add <key> <value>");
                        return;
                    }
                    let key = args[2].clone();
                    let value = args[3].clone();
                    match set_to_redis(con, key.clone(), value.clone()) {
                        Ok(..) => println!("データ登録完了：キー{}, 値={}", key, value),
                        Err(e) => println!("登録失敗：{}", e),
                    }
                }
                "get" => {
                    if args.len() != 3 {
                        eprintln!("usage: cargo run get <key> <value>");
                        return;
                    }
                    let key = args[2].clone();
                    match get_from_redis(con, key.clone()) {
                        Ok(value) => println!("データ取得完了：キー{}, 値={}", key, value),
                        Err(e) => println!("取得失敗：{}", e),
                    }
                }
                _ => {
                    eprintln!("無効なコマンドです。'add' または 'get' を使用してください。");
                }
            }
        }
        Err(e) => {
            eprintln!("Redis接続エラー: {}", e);
        }
    }
}
