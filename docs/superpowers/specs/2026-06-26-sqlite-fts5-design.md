# SQLite + FTS5 迁移设计

## 背景与目标

当前项目使用 PostgreSQL（通过 sea-orm 的 `sqlx-postgres` feature）。需要切换为本地 SQLite，并对 `notes` 表的 `title` + `content` 启用 FTS5 全文检索，使应用可以单文件部署、免外部数据库依赖。

需求：

- 数据库文件放在可执行文件同级目录的 `db/` 子目录下
- 表不存在时自动初始化（幂等启动）
- 仅 `notes(title, content)` 走 FTS5，`tags` 搜索保留现有 LIKE
- 不保留原 PostgreSQL 数据，直接切换

## 影响范围

- `Cargo.toml`（workspace 依赖）
- `crates/db/Cargo.toml`
- `crates/common/src/config.rs`（`DatabaseConfig` 改造）
- `crates/db/src/lib.rs`、`crates/db/src/init.rs`、新增 `crates/db/src/path.rs`
- 新增 `crates/db/migration/` 目录与 5 个迁移文件
- `crates/notes/src/notes/service.rs` 的 `search_notes` 改写
- 新增 `config/default.toml` 配置文件（当前仓库无 `config/` 目录，需创建）

业务 crate（`notebooks`、`tags`、`notes`）的 ORM 调用代码无需改动——它们用的 API 与后端无关。

## 1. 依赖与 feature 调整

### 根 `Cargo.toml`

`[workspace.dependencies]` 新增：

```toml
sea-orm-migration = "2.0.0-rc.41"
```

版本与现有 `sea-orm` 对齐。

### `crates/db/Cargo.toml`

```toml
[dependencies]
common = { path = "../common" }
sea-orm = { workspace = true, features = ["sqlx-sqlite", "runtime-tokio"] }
sea-orm-migration = { workspace = true }
tracing = { workspace = true }
```

把 `sqlx-postgres` 换成 `sqlx-sqlite`，保留 `runtime-tokio`。驱动 feature 集中在 `db` crate（与之前讨论的依赖最小化原则一致）。

### 业务 crate

`notebooks` / `tags` / `notes` 的 `Cargo.toml` 不动。它们只使用 ORM API，不需要驱动 feature。

## 2. 配置瘦身

### `crates/common/src/config.rs`

`DatabaseConfig` 从：

```rust
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}
```

改为：

```rust
pub struct DatabaseConfig {
    pub filename: String,
}
```

SQLite 是单文件数据库，`url` / `max_connections` / `min_connections` 不再适用。

WAL、`foreign_keys`、`synchronous` 等 pragma 在 `init_db` 里硬编码，不暴露为配置项——避免给用户过多旋钮，且这些值在 SQLite 单机场景下有标准推荐。

### 配置文件

`config/default.toml`（或对应位置）：

```toml
[database]
filename = "note.db"
```

## 3. 数据库文件路径解析

新增 `crates/db/src/path.rs`：

```rust
use std::path::PathBuf;

pub fn resolve_db_path(filename: &str) -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().unwrap().join("db");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(filename))
}
```

- 用 `current_exe()` 而非 `current_dir()`，避免从快捷方式/服务启动时 cwd 不可靠
- 自动创建 `db/` 目录
- 调用方拼成 `sqlite://<绝对路径>?mode=rwc`，`mode=rwc` 保证文件不存在时自动创建

## 4. 初始化流程

`crates/db/src/init.rs` 重写：

`crates/db/src/lib.rs` 声明模块：

```rust
pub mod init;
mod path;
mod migration;
```

`crates/db/src/init.rs` 实现：

```rust
use common::common::DbPool;
use common::config::DatabaseConfig;
use common::error::AppError;
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use crate::path;
use crate::migration;

pub async fn init_db(cfg: &DatabaseConfig) -> Result<DbPool, AppError> {
    let path = path::resolve_db_path(&cfg.filename)?;
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let mut options = ConnectOptions::new(url);
    options.sqlx_logging(true);
    let conn = Database::connect(options).await?;

    conn.execute_unprepared("PRAGMA journal_mode=WAL").await?;
    conn.execute_unprepared("PRAGMA foreign_keys=ON").await?;
    conn.execute_unprepared("PRAGMA synchronous=NORMAL").await?;

    migration::Migrator::up(&conn, None).await?;
    info!("数据库初始化成功，路径: {}", path.display());
    Ok(conn)
}
```

### "表不存在才初始化" 的语义保证

`sea-orm-migration` 在数据库里维护 `seaql_migrations` 表，记录已执行的迁移。首次启动：表不存在 → 跑全部迁移 → 记录版本。后续启动：迁移框架看到所有版本已应用 → 直接跳过。这天然满足幂等需求，无需手写 `IF NOT EXISTS` 检查。

## 5. 迁移文件

新建 `crates/db/migration/` 目录，结构：

```
migration/
├── mod.rs                  # 模块声明 + Migrator 定义
├── m20260626_000001_create_notebooks.rs
├── m20260626_000002_create_tags.rs
├── m20260626_000003_create_notes.rs
├── m20260626_000004_create_note_tags.rs
└── m20260626_000005_create_notes_fts.rs
```

### m20260626_000001_create_notebooks

字段对应 `common::models::notebooks::Model`：

- `id` INTEGER PRIMARY KEY AUTOINCREMENT
- `name` TEXT NOT NULL
- `created_at` TEXT (TimeDateTime 在 SQLite 存为 TEXT)
- `updated_at` TEXT

用 `sea_orm_migration::sea_orm::Schema::create_table_from_entity` 反推，或手写 `Manager::create_table()`。手写更可控，推荐手写。

### m20260626_000002_create_tags

- `id` INTEGER PRIMARY KEY AUTOINCREMENT
- `name` TEXT NOT NULL
- `created_at` TEXT

### m20260626_000003_create_notes

- `id` INTEGER PRIMARY KEY AUTOINCREMENT
- `notebook_id` INTEGER NOT NULL, FK→notebooks(id) ON DELETE CASCADE
- `title` TEXT NOT NULL
- `content` TEXT NOT NULL
- `created_at` TEXT
- `updated_at` TEXT

### m20260626_000004_create_note_tags

复合主键（对应 `#[sea_orm(primary_key, auto_increment = false)]`）：

- `note_id` INTEGER NOT NULL, FK→notes(id) ON DELETE CASCADE
- `tag_id` INTEGER NOT NULL, FK→tags(id) ON DELETE CASCADE
- `created_at` TEXT
- PRIMARY KEY (note_id, tag_id)

### m20260626_000005_create_notes_fts

FTS5 虚拟表 + 3 个同步触发器。用外部内容表（external content table）模式避免数据重复：

```sql
CREATE VIRTUAL TABLE notes_fts USING fts5(
    title, content,
    content='notes', content_rowid='id',
    tokenize='trigram'
);

CREATE TRIGGER notes_ai AFTER INSERT ON notes BEGIN
    INSERT INTO notes_fts(rowid, title, content)
    VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER notes_ad AFTER DELETE ON notes BEGIN
    INSERT INTO notes_fts(notes_fts, rowid, title, content)
    VALUES ('delete', old.id, old.title, old.content);
END;

CREATE TRIGGER notes_au AFTER UPDATE ON notes BEGIN
    INSERT INTO notes_fts(notes_fts, rowid, title, content)
    VALUES ('delete', old.id, old.title, old.content);
    INSERT INTO notes_fts(rowid, title, content)
    VALUES (new.id, new.title, new.content);
END;
```

迁移内用 `manager.get_connection().execute_unprepared(sql)` 执行这些语句。

`tokenize='trigram'` 需要 SQLite ≥ 3.34。`rusqlite`/`sqlx-sqlite` 自带的 SQLite 版本足够新，不需要单独确认。

## 6. 搜索代码改写

`crates/notes/src/notes/service.rs` 的 `search_notes` 现状用 `NoteColumn::Title.contains(keyword)`（LIKE），改成走 FTS5：

```rust
use sea_orm::{DbBackend, FromQueryResult, Statement};

#[derive(FromQueryResult)]
struct NoteIdRow { id: i64 }

pub async fn search_notes(
    pool: &DbPool,
    query: SearchNotesQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    let page_num = query.page_num.max(1);
    let page_size = query.page_size.max(1);

    let note_ids: Vec<i64> = if query.keyword.chars().count() < 3 {
        // trigram 至少需要 3 字符，短查询回退到 LIKE
        NoteEntity::find()
            .filter(NoteColumn::Title.contains(query.keyword.as_str()))
            .order_by_desc(NoteColumn::UpdatedAt)
            .all(pool).await?
            .into_iter().map(|n| n.id).collect()
    } else {
        let sql = r#"SELECT n.id FROM notes n
                     JOIN notes_fts f ON f.rowid = n.id
                     WHERE notes_fts MATCH $1
                     ORDER BY n.updated_at DESC"#;
        NoteIdRow::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite, sql, [query.keyword.clone().into()],
        )).all(pool).await?
        .into_iter().map(|r| r.id).collect()
    };

    let total = note_ids.len() as u64;
    let start = ((page_num - 1) * page_size) as usize;
    let page_ids: Vec<i64> = note_ids.into_iter()
        .skip(start).take(page_size as usize).collect();

    let data = if page_ids.is_empty() {
        Vec::new()
    } else {
        let notes = NoteEntity::find()
            .filter(NoteColumn::Id.is_in(page_ids))
            .all(pool).await?;
        notes_with_tags(pool, notes).await?
    };

    Ok(ApiResponse::ok(ApiPageData {
        list: data, page_num, page_size, total,
    }))
}
```

### trigram 短查询回退

trigram 分词器需要至少 3 个字符才能产生 token。`"笔记"`（2 字）走 FTS5 不会命中任何东西。设计上**回退到 LIKE `contains`**——避免"输入一个字搜不到"的体验问题。这是显式选择，不是 bug。

## 7. 不在范围内（YAGNI）

- 不引入 jieba 等外部分词器
- 不实现 `Migrator::down`（开发期不需要回滚）
- 不给 `tags` / `notebooks` 加 FTS5
- 不暴露 SQLite pragma 为配置项
- 不写 PostgreSQL → SQLite 的数据迁移脚本

## 8. 验证方式

1. `cargo build` 通过
2. 删除 `db/` 目录后启动应用，验证 `db/note.db` 自动创建
3. 创建若干笔记，搜索关键词，验证 FTS5 命中
4. 输入 < 3 字符的查询，验证回退到 LIKE 仍能命中
5. 重启应用，验证迁移跳过、表结构保留
