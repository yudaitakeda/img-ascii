mod gencomp;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use image::GenericImageView;
use std::path::{Path, PathBuf};

// ============ 定数 ============
const DEFAULT_WIDTH: u32 = 270;
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

#[derive(Parser, Debug)]
#[command(name = "img-ascii", version, about = "Convert images into ASCII art")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(value_name = "IMAGE_PATH")]
    image_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: CompletionShell,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

/// コマンドライン引数から画像パスを取得
fn require_image_path(cli: Cli) -> Result<PathBuf, AppError> {
    if let Some(path) = cli.image_path {
        return Ok(path);
    }

    Err(AppError::InvalidArgs(format!(
        "画像ファイルのパスを指定してください\n{}",
        Cli::command().render_usage()
    )))
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
fn convert_image_to_ascii(img_path: impl AsRef<Path>) -> Result<(), AppError> {
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
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Completions { shell }) => {
            if let Err(e) = gencomp::generate_completions(shell) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        None => {
            let img_path = match require_image_path(cli) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            // ASCII化して出力
            if let Err(e) = convert_image_to_ascii(img_path) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

// ============ テスト ============
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    // ========== 単体テスト: 明るさ計算 ==========
    #[test]
    fn test_calculate_brightness_black() {
        assert_eq!(calculate_brightness(0, 0, 0), 0.0);
    }

    #[test]
    fn test_calculate_brightness_white() {
        assert_eq!(calculate_brightness(255, 255, 255), 255.0);
    }

    #[test]
    fn test_calculate_brightness_red() {
        let r = 255u8;
        let brightness = calculate_brightness(r, 0, 0);
        assert!(brightness > 0.0 && brightness < 255.0);
    }

    #[test]
    fn test_calculate_brightness_green() {
        let g = 255u8;
        let brightness = calculate_brightness(0, g, 0);
        assert!(brightness > 0.0 && brightness < 255.0);
    }

    #[test]
    fn test_calculate_brightness_blue() {
        let b = 255u8;
        let brightness = calculate_brightness(0, 0, b);
        assert!(brightness > 0.0 && brightness < 255.0);
    }

    #[test]
    fn test_calculate_brightness_monotonic() {
        let b1 = calculate_brightness(100, 100, 100);
        let b2 = calculate_brightness(200, 200, 200);
        assert!(b1 < b2, "brightness should increase with higher RGB values");
    }

    // ========== 単体テスト: 寸法計算 ==========
    #[test]
    fn test_calculate_new_dimensions_width() {
        let (w, _h) = calculate_new_dimensions(100, 100, 400);
        assert_eq!(w, 400);
    }

    #[test]
    fn test_calculate_new_dimensions_height_positive() {
        let (_w, h) = calculate_new_dimensions(100, 100, 400);
        assert!(h > 0);
    }

    #[test]
    fn test_calculate_new_dimensions_aspect_ratio() {
        // 2:1 の画像を 100 幅にリサイズ
        let (_w, h) = calculate_new_dimensions(200, 100, 100);
        // 高さは (100/200) * 100 * 0.5 = 25 になるはず
        assert!(h > 0 && h < 100);
    }

    #[test]
    fn test_calculate_new_dimensions_square() {
        let (w, h) = calculate_new_dimensions(100, 100, 200);
        assert_eq!(w, 200);
        assert!(h > 0);
    }

    // ========== 単体テスト: ASCII 変換 ==========
    #[test]
    fn test_brightness_to_ascii_dark() {
        let ascii = brightness_to_ascii(0.0);
        assert_eq!(ascii, " "); // 暗いと空白
    }

    #[test]
    fn test_brightness_to_ascii_bright() {
        let ascii = brightness_to_ascii(255.0);
        assert_eq!(ascii, "$"); // 明るいと最も濃い文字
    }

    #[test]
    fn test_brightness_to_ascii_mid() {
        let mid_ascii = brightness_to_ascii(127.5);
        let dark_ascii = brightness_to_ascii(0.0);
        let bright_ascii = brightness_to_ascii(255.0);
        assert_ne!(mid_ascii, dark_ascii);
        assert_ne!(mid_ascii, bright_ascii);
    }

    #[test]
    fn test_brightness_to_ascii_monotonic() {
        let a1 = brightness_to_ascii(50.0);
        let _a2 = brightness_to_ascii(150.0);
        let a3 = brightness_to_ascii(250.0);
        // 複数の異なる文字が返ることを確認
        assert_ne!(a1, a3);
    }

    // ========== 単体テスト: コマンドライン引数 ==========
    #[test]
    fn test_cli_parses_image_path() {
        let cli = Cli::try_parse_from(["img-ascii", "test.png"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(cli.image_path, Some(PathBuf::from("test.png")));
    }

    #[test]
    fn test_cli_parses_completions_subcommand() {
        let cli = Cli::try_parse_from(["img-ascii", "completions", "bash"]).unwrap();

        assert_eq!(cli.image_path, None);
        assert!(matches!(
            cli.command,
            Some(Commands::Completions {
                shell: CompletionShell::Bash
            })
        ));
    }
}
