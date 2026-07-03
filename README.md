# img-ascii

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg?logo=rust)
![GitHub last commit](https://img.shields.io/github/last-commit/yudaitakeda/img-ascii)
![GitHub Repo stars](https://img.shields.io/github/stars/yudaitakeda/img-ascii?style=social)
[![Coverage Status](https://coveralls.io/repos/github/yudaitakeda/img-ascii/badge.svg?branch=main)](https://coveralls.io/github/yudaitakeda/img-ascii?branch=main)

## Description

A simple CLI tool to convert images into ASCII art, powered by Rust.
ターミナル上で画像をグレースケール変換し、輝度に基づいた文字割り当てを行うことでASCIIアートを生成します。

## Usage

### 事前準備
Rustがインストールされていることを確認してください。また、変換したい画像ファイルを用意してください。

### 実行方法
リポジトリのルートディレクトリで以下のコマンドを実行します。

```bash
cargo run -- <画像ファイルへのパス>
```

### シェル補完の生成

各シェル向けの補完スクリプトは次のように生成できます。

```bash
cargo run -- completions bash
cargo run -- completions zsh
cargo run -- completions fish
cargo run -- completions powershell
cargo run -- completions elvish
```

### Homebrew 配布

Homebrew tap 用の Formula は [Formula/img-ascii.rb](Formula/img-ascii.rb) に置いてあります。
公開時は release tarball の URL と SHA256 を更新してから tap リポジトリに push してください。

tap 作成後の確認例:

```bash
brew tap <your-user>/img-ascii https://github.com/<your-user>/homebrew-img-ascii
brew install img-ascii
```
