# Rust vs Go: APIサーバーの実装比較

## 概要

このドキュメントでは、Rust（Axum）とGo（Echo）を使用したAPIサーバーの実装を比較し、それぞれの言語・フレームワークの特徴を詳しく解説します。

## プロジェクト構造の比較

### Rust (Axum) プロジェクト構造

```
todo_api/
├── src/
│   ├── domain/          # ドメイン層
│   │   ├── model/       # エンティティ
│   │   └── repository/  # リポジトリインターフェース
│   ├── usecase/         # ユースケース層
│   ├── interface/       # インターフェース層
│   │   ├── gateway/     # 外部システム接続
│   │   └── presenter/   # データ変換
│   └── infrastructure/  # インフラストラクチャ層
│       └── http/        # HTTP関連
├── tests/               # テスト
├── openapi_gen/         # OpenAPI生成コード
└── openapi.yaml         # OpenAPI仕様
```

### Go (Echo) プロジェクト構造

```
todo-api-go/
├── cmd/
│   └── server/
│       └── main.go      # エントリーポイント
├── internal/
│   ├── domain/          # ドメイン層
│   │   ├── model/       # エンティティ
│   │   └── repository/  # リポジトリインターフェース
│   ├── usecase/         # ユースケース層
│   ├── handler/         # HTTPハンドラー
│   └── infrastructure/  # インフラストラクチャ層
├── pkg/                 # パッケージ
├── tests/               # テスト
└── swagger.yaml         # OpenAPI仕様
```

## 言語・フレームワークの特徴比較

### 1. メモリ管理

#### Rust: 所有権システム

```rust
// 所有権の移動
let task = Task::new(1, "Buy groceries".to_string())?;
let description = task.description; // 所有権が移動
// println!("{}", task.description); // コンパイルエラー！

// 借用
let task_ref = &task; // 不変借用
let task_mut_ref = &mut task; // 可変借用
```

#### Go: ガベージコレクション

```go
// ポインタ渡し
task := &Task{ID: 1, Description: "Buy groceries"}
description := task.Description // コピー
fmt.Println(task.Description) // 問題なし

// スライスとマップ
tasks := []*Task{task1, task2}
tasks = append(tasks, task3) // 自動的にメモリ管理
```

### 2. エラーハンドリング

#### Rust: Result型

```rust
// Result型による明示的なエラーハンドリング
pub async fn create_task(&self, create_task: CreateTask) -> Result<Task, TaskError> {
    create_task.validate()?; // ?演算子でエラーを伝播
    self.repository.create(create_task).await
        .map_err(|e| TaskError::Repository(e.to_string()))
}

// エラー型の定義
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found with id: {0}")]
    NotFound(u64),
    #[error("Validation error: {0}")]
    Validation(#[from] TaskValidationError),
}
```

#### Go: 多値返却

```go
// 多値返却によるエラーハンドリング
func (u *TaskUsecase) CreateTask(createTask CreateTask) (*Task, error) {
    if err := createTask.Validate(); err != nil {
        return nil, err
    }
    return u.repository.Create(createTask)
}

// カスタムエラー型
type TaskError struct {
    Code    string
    Message string
}

func (e TaskError) Error() string {
    return fmt.Sprintf("%s: %s", e.Code, e.Message)
}
```

### 3. 並行性

#### Rust: async/await

```rust
// async/awaitによる非同期処理
pub async fn get_all_tasks(&self) -> Result<Vec<Task>, TaskError> {
    self.repository.get_all().await
        .map_err(|e| TaskError::Repository(e.to_string()))
}

// 複数の非同期処理の並行実行
let (tasks, count) = tokio::join!(
    self.repository.get_all(),
    self.repository.count()
);
```

#### Go: goroutine + channel

```go
// goroutineによる並行処理
func (u *TaskUsecase) GetAllTasks() ([]Task, error) {
    tasks, err := u.repository.GetAll()
    if err != nil {
        return nil, err
    }
    return tasks, nil
}

// goroutineとchannelの組み合わせ
func (u *TaskUsecase) GetAllTasksAsync() chan []Task {
    ch := make(chan []Task)
    go func() {
        tasks, _ := u.repository.GetAll()
        ch <- tasks
        close(ch)
    }()
    return ch
}
```

### 4. 型システム

#### Rust: 強力な型システム

```rust
// ジェネリクスとトレイト
pub struct TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    repository: R,
}

// トレイトによる抽象化
pub trait TaskRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Task>, TaskError>;
    async fn create(&self, task: CreateTask) -> Result<Task, TaskError>;
}

// 実装
impl TaskRepository for InMemoryTaskRepository {
    async fn get_all(&self) -> Result<Vec<Task>, TaskError> {
        // 実装
    }
}
```

#### Go: インターフェース

```go
// インターフェースによる抽象化
type TaskRepository interface {
    GetAll() ([]Task, error)
    Create(task CreateTask) (*Task, error)
}

// 構造体
type TaskUsecase struct {
    repository TaskRepository
}

// 実装
type InMemoryTaskRepository struct {
    tasks map[uint64]*Task
    mu    sync.RWMutex
}

func (r *InMemoryTaskRepository) GetAll() ([]Task, error) {
    r.mu.RLock()
    defer r.mu.RUnlock()
    // 実装
}
```

## HTTPハンドラーの実装比較

### Rust (Axum)

```rust
// OpenAPI生成コードを使用したハンドラー
#[async_trait]
impl<T> Tasks<ApiError> for TaskApiImpl<T>
where
    T: crate::usecase::task::TaskUsecase + Send + Sync + 'static,
{
    async fn tasks_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<TasksGetResponse, ApiError> {
        let domain_tasks = self.usecase.get_all_tasks().await?;
        let api_tasks = TaskMapper::domain_vec_to_api(domain_tasks);
        Ok(TasksGetResponse::Status200_ListOfAllTasks(api_tasks))
    }

    async fn tasks_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        body: &openapi::models::CreateTask,
    ) -> Result<TasksPostResponse, ApiError> {
        let domain_create = TaskMapper::api_create_to_domain(body.clone())?;
        let domain_task = self.usecase.create_task(domain_create).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksPostResponse::Status201_TaskCreatedSuccessfully(api_task))
    }
}
```

### Go (Echo)

```go
// Echoフレームワークを使用したハンドラー
type TaskHandler struct {
    usecase TaskUsecase
}

func NewTaskHandler(usecase TaskUsecase) *TaskHandler {
    return &TaskHandler{usecase: usecase}
}

func (h *TaskHandler) GetTasks(c echo.Context) error {
    tasks, err := h.usecase.GetAllTasks()
    if err != nil {
        return c.JSON(http.StatusInternalServerError, map[string]string{
            "error": err.Error(),
        })
    }
    return c.JSON(http.StatusOK, tasks)
}

func (h *TaskHandler) CreateTask(c echo.Context) error {
    var createTask CreateTask
    if err := c.Bind(&createTask); err != nil {
        return c.JSON(http.StatusBadRequest, map[string]string{
            "error": "Invalid request body",
        })
    }

    task, err := h.usecase.CreateTask(createTask)
    if err != nil {
        return c.JSON(http.StatusInternalServerError, map[string]string{
            "error": err.Error(),
        })
    }

    return c.JSON(http.StatusCreated, task)
}

// ルーティング設定
func setupRoutes(e *echo.Echo, handler *TaskHandler) {
    e.GET("/tasks", handler.GetTasks)
    e.POST("/tasks", handler.CreateTask)
    e.GET("/tasks/:id", handler.GetTask)
    e.PUT("/tasks/:id", handler.UpdateTask)
    e.DELETE("/tasks/:id", handler.DeleteTask)
}
```

## パフォーマンス比較

### メモリ使用量

| 項目 | Rust | Go |
|------|------|----|
| **メモリ使用量** | 低い（GCなし） | 中程度（GCあり） |
| **メモリ予測可能性** | 高い | 中程度 |
| **メモリリーク** | コンパイル時に防止 | ランタイムで検出 |

### 実行速度

| 項目 | Rust | Go |
|------|------|----|
| **起動時間** | 長い | 短い |
| **実行速度** | 非常に高速 | 高速 |
| **コンパイル時間** | 長い | 短い |

### 並行処理

| 項目 | Rust | Go |
|------|------|----|
| **並行処理モデル** | async/await | goroutine |
| **スレッド管理** | 手動 | 自動 |
| **メモリ安全性** | コンパイル時保証 | ランタイム保証 |

## 開発体験の比較

### 学習曲線

#### Rust
- **急峻な学習曲線**
- 所有権システムの理解が必要
- 借用チェッカーの概念
- 強力な型システム

#### Go
- **緩やかな学習曲線**
- シンプルな文法
- 直感的な並行処理
- 豊富な標準ライブラリ

### 開発効率

#### Rust
- **コンパイル時のエラー検出**
- リファクタリングの安全性
- 優れたIDEサポート
- 詳細なエラーメッセージ

#### Go
- **高速なコンパイル**
- シンプルな文法
- 豊富なツールチェーン
- 活発なコミュニティ

### デバッグ

#### Rust
- **コンパイル時の問題解決**
- ランタイムエラーが少ない
- 詳細なスタックトレース
- 優れたデバッガーサポート

#### Go
- **ランタイムでの問題解決**
- 豊富なデバッグツール
- pprofによるプロファイリング
- 簡単なデバッグ

## 適している用途

### Rustが適している場合

- **高性能が要求されるシステム**
- **メモリ安全性が重要なシステム**
- **システムプログラミング**
- **組み込みシステム**
- **WebAssembly**

### Goが適している場合

- **マイクロサービス**
- **APIサーバー**
- **DevOpsツール**
- **プロトタイピング**
- **チーム開発**

## まとめ

### Rustの利点

1. **メモリ安全性**: コンパイル時に保証
2. **パフォーマンス**: ゼロコスト抽象化
3. **型安全性**: 強力な型システム
4. **並行性**: 安全な並行処理

### Goの利点

1. **開発速度**: 高速なコンパイル
2. **学習容易性**: シンプルな文法
3. **豊富なエコシステム**: 標準ライブラリ
4. **並行処理**: 簡単なgoroutine

### 選択の指針

- **パフォーマンスと安全性が最優先**: Rust
- **開発速度とチーム生産性が最優先**: Go
- **既存のスキルセット**: チームの経験を考慮
- **プロジェクトの要件**: 具体的なニーズに応じて選択

両言語とも優れた選択肢であり、プロジェクトの要件とチームの状況に応じて適切に選択することが重要です。 