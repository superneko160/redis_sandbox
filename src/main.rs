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

// コマンドライン引数を解析
fn parse_args() -> Result<(String, Vec<String>), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("引数が不足しています".to_string());
    }

    let command = args[1].clone();
    let params = args[2..].to_vec();

    Ok((command, params))
}

// 実行方法の表示
fn show_usage() {
    eprintln!("usage:");
    eprintln!("  cargo run add <key> <value>");
    eprintln!("  cargo run get <key>");
}

// コマンドの実行
fn execute_command(con: redis::Connection, command: &str, params: &[String]) -> Result<(), String> {
    match command {
        "add" => execute_add_command(con, params),
        "get" => execute_get_command(con, params),
        _  => Err("無効なコマンドです。'add' または 'get' を使用してください。".to_string())
    }
}

// addコマンドの実行
fn execute_add_command(con: redis::Connection, params: &[String]) -> Result<(), String> {
    if params.len() != 2 {
        return Err("usage: cargo run add <key> <value>".to_string());
    }
    
    let key = params[0].clone();
    let value = params[1].clone();

    match set_to_redis(con, key.clone(), value.clone()) {
        Ok(..) => {
            println!("データ登録完了：キー{}, 値={}", key, value);
            Ok(())
        }
        Err(e) => {
            Err(format!("登録失敗: {}", e))
        }
    }
}

// getコマンドの実行
fn execute_get_command(con: redis::Connection, params: &[String]) -> Result<(), String> {
    if params.len() != 1 {
        return Err("usage: cargo run get <key> <value>".to_string());
    }
    
    let key = params[0].clone();
    
    match get_from_redis(con, key.clone()) {
        Ok(value) => {
            println!("データ取得完了：キー{}, 値={}", key, value);
            Ok(())
        }
        Err(e) => {
            Err(format!("取得失敗: {}", e))
        }
    }
}

// アプリケーションの実行
fn run() -> Result<(), String> {
    let (command, params) = parse_args().map_err(|_| {
        show_usage();
        "".to_string()
    })?;

    let con = connection_handling().map_err(|e| {
        format!("Redis接続エラー: {}", e)
    })?;

    execute_command(con, &command, &params)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("エラー: {}", e);
        std::process::exit(1);
    }
}

