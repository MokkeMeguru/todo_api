# はじめに

## 概要

このプロジェクトは、Rust言語とAxumフレームワークを使用したTodo APIサーバーのサンプルです。クリーンアーキテクチャの原則に従って設計され、OpenAPI仕様からコードを自動生成するアプローチを採用しています。

## 前提条件

- Rust 1.70以上
- Cargo（Rustのパッケージマネージャー）

## インストール

### 1. Rustのインストール

```bash
# Rustupを使用してRustをインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 環境変数を設定
source ~/.cargo/env

# バージョン確認
rustc --version
cargo --version
```

### 2. プロジェクトのクローン

```bash
git clone <repository-url>
cd todo_api
```

## ビルドと実行

### 1. 依存関係のインストール

```bash
cargo build
```

### 2. サーバーの起動

```bash
cargo run --bin api
```

サーバーは `http://localhost:3000` で起動します。

### 3. テストの実行

```bash
# 全テストを実行
cargo test

# 特定のテストを実行
cargo test test_name

# テストの詳細出力
cargo test -- --nocapture
```

## APIの使用例

### 1. タスクの作成

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"description": "Buy groceries"}'
```

### 2. タスクの取得

```bash
# 全タスクを取得
curl http://localhost:3000/tasks

# 特定のタスクを取得
curl http://localhost:3000/tasks/1
```

### 3. タスクの更新

```bash
curl -X PUT http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"description": "Buy milk and bread", "completed": true}'
```

### 4. タスクの削除

```bash
curl -X DELETE http://localhost:3000/tasks/1
```

### 5. タスクの完了/未完了

```bash
# タスクを完了にする
curl -X PUT http://localhost:3000/tasks/1/complete

# タスクを未完了にする
curl -X PUT http://localhost:3000/tasks/1/uncomplete
```

### 6. タスクの検索

```bash
# 完了したタスクを取得
curl http://localhost:3000/tasks/completed

# 未完了のタスクを取得
curl http://localhost:3000/tasks/pending

# キーワードで検索
curl "http://localhost:3000/tasks/search?q=grocery"
```

## 開発環境のセットアップ

### 1. IDEの設定

#### VS Code

推奨拡張機能：
- `rust-analyzer` - Rust言語サポート
- `crates` - Cargo.tomlの依存関係管理
- `CodeLLDB` - デバッグサポート

#### IntelliJ IDEA / CLion

- Rustプラグインをインストール
- Cargoプロジェクトとして開く

### 2. コードフォーマット

```bash
# コードの自動フォーマット
cargo fmt

# リンターの実行
cargo clippy
```

### 3. ドキュメントの生成

```bash
# APIドキュメントの生成
cargo doc --open
```

## プロジェクト構造

```
todo_api/
├── src/
│   ├── domain/          # ドメイン層
│   ├── usecase/         # ユースケース層
│   ├── interface/       # インターフェース層
│   └── infrastructure/  # インフラストラクチャ層
├── tests/               # テスト
├── docs/                # ドキュメント
├── openapi_gen/         # OpenAPI生成コード
└── openapi.yaml         # OpenAPI仕様
```

## トラブルシューティング

### よくある問題

#### 1. コンパイルエラー

```bash
# 依存関係の更新
cargo update

# キャッシュのクリア
cargo clean
cargo build
```

#### 2. ポートが使用中

```bash
# 使用中のポートを確認
lsof -i :3000

# プロセスを終了
kill -9 <PID>
```

#### 3. 権限エラー

```bash
# 権限を確認
ls -la

# 必要に応じて権限を変更
chmod +x target/debug/api
```

## 次のステップ

### 1. 機能の追加

- 新しいエンドポイントの追加
- データベース統合
- 認証・認可の実装

### 2. テストの拡張

- 統合テストの追加
- パフォーマンステストの実装
- E2Eテストの追加

### 3. デプロイ

- Docker化
- CI/CDパイプラインの構築
- クラウドへのデプロイ

## 参考資料

- [Rust公式ドキュメント](https://doc.rust-lang.org/)
- [Axum公式ドキュメント](https://docs.rs/axum/)
- [OpenAPI仕様](https://swagger.io/specification/)
- [クリーンアーキテクチャ](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

## 貢献

このプロジェクトへの貢献を歓迎します。以下の手順でお願いします：

1. フォークを作成
2. フィーチャーブランチを作成
3. 変更をコミット
4. プルリクエストを作成

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。 