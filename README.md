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

## Commit Rules

* 形式: `type: 内容`

例:
* feat: ASCIIアート出力機能を実装
* fix: ファイルパスのエラーを修正
* refactor: コードの構造を整理
