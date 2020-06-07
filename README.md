# network-examples

Rustでネットワークのパケットキャプチャ・解析・制御等を実装するための勉強用レポジトリです。

## 目標

- [ ] パケットキャプチャ
- [ ] フロー解析 / 特徴抽出（保存）
- [ ] 特定のアクセス制御

## 知識

### ネットワーク

- [パケット \- Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%91%E3%82%B1%E3%83%83%E3%83%88)

### Rust

- [Rust公式](https://www.rust-lang.org/)

## その他

### 実行方法

ソースコードのクローン
```sh
# クローン
git clone https://github.com/telumo/network-examples.git

# 移動
cd network-examples

# ブランチの作成
git branch <name>
# ex) git branch hasegawa

# チェックアウト

git checkout <name>
# ex)  git chechout hasegawa
```

ビルド | 実行
```
# ビルド
cargo build

# 実行
cargo run
```

コミット　& プッシュ
```
# コミット
git add .
git commit -m "<メッセージ>"

# 最新版の取得
git checkout master
git pull
git checkout <name>

# マージ
git merge master

# プッシュ
git push  
# 初回時のみ
git push --set-upstream origin <name>
```

プルリクエスト & マージ

1. githubのプルリクエスト機能で master <= <name>ブランチ にプルリクエストを送る。
2. マージする


### 環境構築（Cargo）

#### インストール

```sh

# インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# アクティベート
source ~/.cargo/env

# update
rustup update

```
[Rustのインストール\-Rustプログラミング言語](https://www.rust-lang.org/tools/install)

#### 実行

```sh

# ビルド
cargo build

# 実行
cargo run
```

## 問題

***数字をて足す***

- `numbers.txt`ファイルから数字を取得する
numbers.txt
```txt
1
2
```
- 全ての数字の和を求める

- `numbers.txt`に追記する
```txt
1
2
3
```







