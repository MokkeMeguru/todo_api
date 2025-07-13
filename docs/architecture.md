# Todo API アーキテクチャドキュメント

## 概要

このプロジェクトは、クリーンアーキテクチャの原則に従って設計されたTodo APIです。Rust言語とAxumフレームワークを使用し、OpenAPI仕様からコードを自動生成するアプローチを採用しています。

## アーキテクチャ概要

このプロジェクトは以下の4つの主要なレイヤーで構成されています：

```
┌─────────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                     │
│  (HTTP, Server, Database, External Services)               │
├─────────────────────────────────────────────────────────────┤
│                    Interface Layer                          │
│  (Controllers, Gateways, Adapters)                         │
├─────────────────────────────────────────────────────────────┤
│                    Usecase Layer                            │
│  (Business Logic, Application Services)                    │
├─────────────────────────────────────────────────────────────┤
│                    Domain Layer                             │
│  (Entities, Value Objects, Repository Interfaces)          │
└─────────────────────────────────────────────────────────────┘
```

## ディレクトリ構造

```
todo_api/
├── src/
│   ├── domain/                 # ドメイン層
│   │   ├── model/             # エンティティとバリューオブジェクト
│   │   │   └── task.rs        # Taskエンティティ
│   │   └── repository/        # リポジトリインターフェース
│   │       └── task.rs        # TaskRepositoryトレイト
│   ├── usecase/               # ユースケース層
│   │   └── task.rs            # ビジネスロジック
│   ├── interface/             # インターフェース層
│   │   ├── gateway/           # 外部システムとの接続
│   │   │   └── inmemory/      # インメモリ実装
│   │   │       └── task.rs    # InMemoryTaskRepository
│   │   └── presenter/         # データ変換
│   │       └── task.rs        # TaskMapper
│   └── infrastructure/        # インフラストラクチャ層
│       ├── http/              # HTTP関連
│       │   ├── api_impl.rs    # OpenAPI生成コードの実装
│       │   ├── generated_routes.rs # 生成されたルーター
│       │   └── handlers/      # HTTPハンドラー
│       └── server.rs          # サーバー起動
├── cmd/
│   └── api/
│       └── main.rs            # エントリーポイント
├── openapi_gen/               # OpenAPI生成コード
├── openapi.yaml               # OpenAPI仕様
└── Cargo.toml                 # 依存関係
```

## レイヤー詳細

### 1. Domain Layer (`src/domain/`)

ドメイン層は、ビジネスの核心となるエンティティ、バリューオブジェクト、およびリポジトリインターフェースを定義します。

#### モデル (`src/domain/model/`)

- **Task**: タスクエンティティ
  - バリデーションロジック
  - ビジネスルール（完了/未完了の状態管理）
  - 不変性の保証

- **CreateTask/UpdateTask**: タスク作成・更新用のDTO
  - 入力バリデーション
  - ドメインルールの適用

#### リポジトリ (`src/domain/repository/`)

- **TaskRepository**: タスク永続化の抽象化
  - 依存性逆転の原則に従ったインターフェース
  - テスト可能な設計（Mockall使用）

### 2. Usecase Layer (`src/usecase/`)

アプリケーションのビジネスロジックを実装します。

- **TaskUsecase**: タスク操作のユースケーストレイト
- **TaskUsecaseImpl**: 具体的なビジネスロジックの実装
  - タスクのCRUD操作
  - 検索機能
  - ステータス管理

### 3. Interface Layer (`src/interface/`)

外部システムとの接続を管理します。

#### Gateway (`src/interface/gateway/`)

- **InMemoryTaskRepository**: インメモリ実装
  - 開発・テスト用の軽量実装
  - スレッドセーフな設計（Arc<Mutex<>>）

#### Presenter (`src/interface/presenter/`)

- **TaskMapper**: ドメイン⇔APIモデル変換
  - データの方向に合わせた変換
  - 型安全性の保証

### 4. Infrastructure Layer (`src/infrastructure/`)

技術的な実装詳細を扱います。

#### HTTP (`src/infrastructure/http/`)

- **api_impl.rs**: OpenAPI生成コードの実装
  - ドメイン層との橋渡し
  - エラーハンドリング
- **generated_routes.rs**: 自動生成されたルーター
- **handlers/**: HTTPリクエスト処理

#### Server (`src/infrastructure/server.rs`)

- Axumサーバーの設定と起動
- 依存性注入の設定

## OpenAPI統合

### 自動生成アプローチ

1. **openapi.yaml**: API仕様の定義
2. **openapi_gen/**: OpenAPI Generatorで生成されたコード
3. **generated_routes.rs**: 自動生成されたルーターの統合

### 利点

- API仕様とコードの同期
- 型安全性の向上
- ドキュメント自動生成
- クライアントコードの自動生成

## 依存性の方向

```
Infrastructure → Interface → Usecase → Domain
     ↑              ↑           ↑
     └──────────────┴───────────┘
       依存性注入による逆転
```

- 内側のレイヤーは外側のレイヤーに依存しない
- 依存性注入により、テストと保守性が向上
- トレイトを使用した抽象化

## エラーハンドリング

各レイヤーで適切なエラー型を定義：

- **Domain**: `TaskValidationError`
- **Repository**: `TaskError`
- **Usecase**: `TaskError`
- **API**: `ApiError`

## テスト戦略

- **Unit Tests**: 各レイヤーでの単体テスト
- **Integration Tests**: レイヤー間の統合テスト
- **Mock Testing**: Mockallを使用したモックテスト

## 技術スタック

- **Language**: Rust 2021
- **Web Framework**: Axum
- **Serialization**: Serde
- **Validation**: Validator
- **Testing**: Mockall
- **Documentation**: Utoipa (Swagger)
- **Code Generation**: OpenAPI Generator

## 設計原則

1. **クリーンアーキテクチャ**: 依存性の方向を制御
2. **SOLID原則**: 単一責任、開放閉鎖、依存性逆転
3. **DRY**: 重複コードの排除
4. **KISS**: シンプルな設計
5. **テストファースト**: テスト可能な設計

## 今後の拡張可能性

- **データベース統合**: PostgreSQL、MySQL等
- **認証・認可**: JWT、OAuth2
- **キャッシュ**: Redis
- **メッセージング**: RabbitMQ、Kafka
- **監視**: Prometheus、Grafana
- **コンテナ化**: Docker、Kubernetes 