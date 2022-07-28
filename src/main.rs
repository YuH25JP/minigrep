extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // String型のvectorにコマンドライン引数を格納
    // args[0]にはコマンド自体のパスが格納され，与えられたコマンドライン引数は[1]以降に入る
    let args: Vec<String> = env::args().collect();

    // Config::new()の返り値(Result型)がOkのときはunwrap，そうじゃないときはエラーメッセージを表示してプロセスを終了
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    // if let ... match制御において，ある1つの場合だけ処理を行いたいときに分を簡潔にできる
    // 参照：https://doc.rust-jp.rs/book-ja/ch06-03-if-let.html
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}