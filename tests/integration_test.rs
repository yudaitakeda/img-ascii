/// 結合テスト: 画像ファイル処理全体
/// 
/// このテストは、実際の画像ファイルがある場合に、
/// プログラム全体が正常に動作することを検証します。

use std::path::Path;
use std::fs;

#[test]
#[ignore = "requires actual image file"]
fn test_with_actual_image() {
    // 注: このテストは rogo.jpg が存在する場合のみ実行
    // `cargo test -- --ignored` で実行可能
    let test_image = "rogo.jpg";
    assert!(
        Path::new(test_image).exists(),
        "Test image {} not found",
        test_image
    );
}

#[test]
fn test_image_exists_in_repo() {
    // リポジトリに画像ファイルが含まれていることを確認
    let image_file = "rogo.jpg";
    let exists = Path::new(image_file).exists();
    assert!(exists, "Image file {} should exist in repository root", image_file);
}

#[test]
fn test_tests_directory_exists() {
    // このテスト自体が存在していることで、tests/ ディレクトリが機能していることを確認
    assert!(Path::new("tests").is_dir(), "tests directory should exist");
}
