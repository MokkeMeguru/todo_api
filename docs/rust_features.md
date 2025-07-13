# Rust API サーバーの特徴と基本書き方

## 概要

このプロジェクトは、Rust言語を使用したAPIサーバーのサンプルです。Rustの特徴的な機能を活用し、型安全性、メモリ安全性、並行性を重視した設計になっています。

## Rust特有の機能

### 1. 所有権システム (Ownership System)

Rustの最も特徴的な機能である所有権システムは、メモリ安全性を保証します。

```rust
// Rust: 所有権の移動
let task = Task::new(1, "Buy groceries".to_string())?;
let task_description = task.description; // task.descriptionの所有権が移動
// println!("{}", task.description); // コンパイルエラー！所有権が移動済み

// Go: ポインタ渡し
// task := Task{ID: 1, Description: "Buy groceries"}
// description := task.Description // コピー
// fmt.Println(task.Description) // 問題なし
```

### 2. 借用チェッカー (Borrow Checker)

コンパイル時にデータ競合を防ぎます。

```rust
// Rust: 借用チェッカーによる安全性
let mut tasks = vec![task1, task2];
let first_task = &tasks[0]; // 不変借用
// tasks.push(task3); // コンパイルエラー！可変借用と不変借用の競合

// Go: ランタイムでの競合検出
// tasks := []Task{task1, task2}
// firstTask := &tasks[0]
// tasks = append(tasks, task3) // 問題なし（ただし、firstTaskの参照が無効になる可能性）
```

### 3. トレイト (Traits)

インターフェースの概念を実現し、多態性を提供します。

```rust
// Rust: トレイトによる抽象化
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

// Go: インターフェース
// type TaskRepository interface {
//     GetAll() ([]Task, error)
//     Create(task CreateTask) (Task, error)
// }
```

### 4. エラーハンドリング

`Result<T, E>`型を使用した明示的なエラーハンドリング。

```rust
// Rust: Result型によるエラーハンドリング
pub async fn create_task(&self, create_task: CreateTask) -> Result<Task, TaskError> {
    create_task.validate()?; // ?演算子でエラーを伝播
    self.repository.create(create_task).await
        .map_err(|e| TaskError::Repository(e.to_string()))
}

// Go: 多値返却によるエラーハンドリング
// func (u *TaskUsecase) CreateTask(createTask CreateTask) (Task, error) {
//     if err := createTask.Validate(); err != nil {
//         return Task{}, err
//     }
//     return u.repository.Create(createTask)
// }
```

### 5. 非同期プログラミング

`async/await`による非同期処理。

```rust
// Rust: async/await
pub async fn get_all_tasks(&self) -> Result<Vec<Task>, TaskError> {
    self.repository.get_all().await
        .map_err(|e| TaskError::Repository(e.to_string()))
}

// Go: goroutine + channel
// func (u *TaskUsecase) GetAllTasks() ([]Task, error) {
//     tasks, err := u.repository.GetAll()
//     if err != nil {
//         return nil, err
//     }
//     return tasks, nil
// }
```

## 基本的な書き方

### 1. 構造体の定義

```rust
// Rust: 構造体とメソッド
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn new(id: u64, description: String) -> Result<Self, TaskValidationError> {
        let now = chrono::Utc::now();
        let task = Self {
            id,
            description: description.clone(),
            completed: false,
            created_at: now,
            updated_at: now,
        };
        task.validate()?;
        Ok(task)
    }
}

// Go: 構造体とメソッド
// type Task struct {
//     ID          uint64    `json:"id"`
//     Description string    `json:"description"`
//     Completed   bool      `json:"completed"`
//     CreatedAt   time.Time `json:"created_at"`
//     UpdatedAt   time.Time `json:"updated_at"`
// }
//
// func NewTask(id uint64, description string) (*Task, error) {
//     now := time.Now().UTC()
//     task := &Task{
//         ID:          id,
//         Description: description,
//         Completed:   false,
//         CreatedAt:   now,
//         UpdatedAt:   now,
//     }
//     return task, task.Validate()
// }
```

### 2. エラー型の定義

```rust
// Rust: thiserrorクレートを使用したエラー型
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found with id: {0}")]
    NotFound(u64),
    #[error("Validation error: {0}")]
    Validation(#[from] TaskValidationError),
    #[error("Repository error: {0}")]
    Repository(String),
}

// Go: カスタムエラー型
// type TaskError struct {
//     Code    string
//     Message string
// }
//
// func (e TaskError) Error() string {
//     return fmt.Sprintf("%s: %s", e.Code, e.Message)
// }
```

### 3. 依存性注入

```rust
// Rust: ジェネリクスとトレイトによる依存性注入
pub struct TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    repository: R,
}

impl<R> TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

// Go: インターフェースによる依存性注入
// type TaskUsecase struct {
//     repository TaskRepository
// }
//
// func NewTaskUsecase(repository TaskRepository) *TaskUsecase {
//     return &TaskUsecase{repository: repository}
// }
```

### 4. HTTPハンドラーの実装

```rust
// Rust: Axumフレームワーク
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
}

// Go: Echoフレームワーク
// func (h *TaskHandler) GetTasks(c echo.Context) error {
//     tasks, err := h.usecase.GetAllTasks()
//     if err != nil {
//         return c.JSON(http.StatusInternalServerError, map[string]string{
//             "error": err.Error(),
//         })
//     }
//     return c.JSON(http.StatusOK, tasks)
// }
```

## Rust vs Go の比較

| 特徴 | Rust | Go |
|------|------|----|
| **メモリ管理** | 所有権システム（コンパイル時） | GC（ランタイム） |
| **並行性** | async/await + 所有権 | goroutine + channel |
| **エラーハンドリング** | Result<T, E>型 | 多値返却 |
| **型システム** | 強力な型推論 + トレイト | インターフェース |
| **パフォーマンス** | ゼロコスト抽象化 | GCオーバーヘッド |
| **学習曲線** | 急峻 | 緩やか |
| **コンパイル時間** | 長い | 短い |
| **ランタイム** | なし | あり |

## このプロジェクトでの活用例

### 1. 型安全性の活用

```rust
// ドメインモデルでの型安全性
pub struct CreateTask {
    pub description: String,
}

impl CreateTask {
    pub fn new(description: String) -> Result<Self, TaskValidationError> {
        let create_task = Self { description };
        create_task.validate()?;
        Ok(create_task)
    }
}
```

### 2. 非同期処理の活用

```rust
// リポジトリでの非同期処理
#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn get_all(&self) -> Result<Vec<Task>, TaskError> {
        let tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        Ok(tasks.values().cloned().collect())
    }
}
```

### 3. トレイトによる抽象化

```rust
// ユースケースでのトレイト活用
pub trait TaskUsecase: Send + Sync {
    fn get_all_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
    fn create_task<'a>(&'a self, create_task: CreateTask) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
}
```

## まとめ

RustのAPIサーバーは、以下の特徴により堅牢で高性能なアプリケーションを構築できます：

1. **コンパイル時の安全性保証**
2. **ゼロコスト抽象化**
3. **強力な型システム**
4. **明示的なエラーハンドリング**
5. **効率的な非同期処理**

これらの特徴により、本番環境でも安心して運用できるAPIサーバーを構築できます。 