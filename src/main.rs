use std::env;
use image::GenericImageView;

// ============ 定数 ============
const DEFAULT_WIDTH: u32 = 200;
const HEIGHT_RATIO: f32 = 0.5;
const BRIGHTNESS_R: f32 = 0.299;
const BRIGHTNESS_G: f32 = 0.587;
const BRIGHTNESS_B: f32 = 0.114;
const MAX_BRIGHTNESS: f32 = 255.0;

const ASCII_CHARS: &[&str] = &[
    " ", ".", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i",
    "~", "+", "_", "-", "?", "]", "[", "}", "{", "1", ")", "(",
    "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u", "v", "c",
    "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m",
    "w", "q", "p", "d", "b", "k", "h", "a", "o", "*", "#", "M",
    "W", "&", "8", "%", "B", "@", "$"
];

const USAGE: &str = "使用法: cargo run -- <画像へのパス>";

// ============ エラー型 ============
#[derive(Debug)]
enum AppError {
    InvalidArgs(String),
    ImageLoadError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::InvalidArgs(msg) => write!(f, "引数エラー: {}", msg),
            AppError::ImageLoadError(msg) => write!(f, "画像読み込みエラー: {}", msg),
        }
    }
}

// ============ 関数群 ============

/// コマンドライン引数から画像パスを取得
fn get_image_path(args: &[String]) -> Result<String, AppError> {
    if args.len() < 2 {
        return Err(AppError::InvalidArgs(USAGE.to_string()));
    }
    Ok(args[1].clone())
}

/// 画像の新しいサイズを計算
fn calculate_new_dimensions(width: u32, height: u32, target_width: u32) -> (u32, u32) {
    let aspect_ratio = height as f32 / width as f32;
    let new_height = ((aspect_ratio * target_width as f32) * HEIGHT_RATIO) as u32;
    (target_width, new_height)
}

/// ピクセルの明るさを計算
fn calculate_brightness(r: u8, g: u8, b: u8) -> f32 {
    BRIGHTNESS_R * r as f32 + BRIGHTNESS_G * g as f32 + BRIGHTNESS_B * b as f32
}

/// 明るさに対応するASCII文字を取得
fn brightness_to_ascii(brightness: f32) -> &'static str {
    let normalized = brightness / MAX_BRIGHTNESS;
    let index = (normalized * (ASCII_CHARS.len() - 1) as f32).round() as usize;
    ASCII_CHARS[index.min(ASCII_CHARS.len() - 1)]
}

/// 画像をASCII化して出力
fn convert_image_to_ascii(img_path: &str) -> Result<(), AppError> {
    // 画像を開く
    let img = image::open(img_path)
        .map_err(|e| AppError::ImageLoadError(e.to_string()))?;

    let (width, height) = img.dimensions();
    let (new_width, new_height) = calculate_new_dimensions(width, height, DEFAULT_WIDTH);

    let resized_img = img.thumbnail(new_width, new_height);

    // ASCII画像を生成して出力
    for y in 0..resized_img.height() {
        for x in 0..resized_img.width() {
            let pixel = resized_img.get_pixel(x, y);

            let brightness = calculate_brightness(pixel[0], pixel[1], pixel[2]);
            let ascii_char = brightness_to_ascii(brightness);

            print!("{}", ascii_char);
        }
        println!();
    }

    Ok(())
}

// ============ メイン処理 ============

fn main() {
    let args: Vec<String> = env::args().collect();

    // 画像パスを取得
    let img_path = match get_image_path(&args) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // ASCII化して出力
    if let Err(e) = convert_image_to_ascii(&img_path) {
        eprintln!("{}", e);
    }
}

// ============ テスト ============
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_brightness() {
        // 黒 (0, 0, 0)
        assert_eq!(calculate_brightness(0, 0, 0), 0.0);
        // 白 (255, 255, 255)
        assert_eq!(calculate_brightness(255, 255, 255), 255.0);
    }

    #[test]
    fn test_new_dimensions() {
        let (w, h) = calculate_new_dimensions(100, 100, 400);
        assert_eq!(w, 400);
        assert!(h > 0);
    }

    #[test]
    fn test_brightness_to_ascii() {
        let dark = brightness_to_ascii(0.0);
        let light = brightness_to_ascii(255.0);
        assert_ne!(dark, light);
    }
}
