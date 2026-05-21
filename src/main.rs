use std::env;
use image::GenericImageView;

fn main() {
    // 1. コマンドライン引数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("使用法: cargo run -- <画像へのパス>");
        return;
    }
    let img_path = &args[1];

    // 2. 画像を開く
    let img = image::open(img_path)
        .expect("画像を開けませんでした。パスを確認してください。");

    // 3. 高解像度に設定（ここがポイント）
    let (width, height) = img.dimensions();
    let new_width = 400; // ← 100 → 200にアップ
    let new_height = ((height as f32 / width as f32) * new_width as f32 * 0.5) as u32;

    let resized_img = img.thumbnail(new_width, new_height);

    // 4. 高階調ASCII（超重要）
    let ascii_chars = [
        " ", ".", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i",
        "~", "+", "_", "-", "?", "]", "[", "}", "{", "1", ")", "(",
        "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u", "v", "c",
        "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m",
        "w", "q", "p", "d", "b", "k", "h", "a", "o", "*", "#", "M",
        "W", "&", "8", "%", "B", "@", "$"
    ];

    // 5. 描画
    for y in 0..resized_img.height() {
        for x in 0..resized_img.width() {
            let pixel = resized_img.get_pixel(x, y);

            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            // 人間の視覚に近い明るさ計算
            let brightness = 0.299 * r + 0.587 * g + 0.114 * b;

            // なめらかに変換
            let index = ((brightness / 255.0)
                * (ascii_chars.len() - 1) as f32)
                .round() as usize;

            print!("{}", ascii_chars[index]);
        }
        println!();
    }
}
