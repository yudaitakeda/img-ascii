/// 使用例: 基本的な画像から ASCII への変換
///
/// 実行方法:
/// ```
/// cargo run --example basic_convert -- rogo.jpg
/// ```

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("使用法: {} <画像へのパス>", args[0]);
        std::process::exit(1);
    }

    let img_path = &args[1];

    // ファイルの存在確認
    if !Path::new(img_path).exists() {
        eprintln!("エラー: ファイル '{}' が見つかりません", img_path);
        std::process::exit(1);
    }

    println!("画像ファイル: {}", img_path);
    println!("ASCII 変換を開始します...\n");

    // ここで実際の img-ascii ライブラリを使用するとよいです
    // 現在のコードでは main.rs が library として使用できないため、
    // 別プロセスで実行する方法もあります
}
