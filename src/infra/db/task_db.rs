//! Author: xiaoYown
//! Created: 2026-03-04
//! Description: SQLite task metadata database

use anyhow::Result;
use rusqlite::{params, Connection};

use crate::core::executor::types::{TaskMetadata, TaskStatus};
use crate::shared::constants::{HOST_LOG_DIR, TASK_DB_FILE};

/// 分页参数
pub struct PageParams {
    pub page: usize,
    pub size: usize,
}

/// 任务过滤条件
pub struct TaskFilter {
    pub space_name: Option<String>,
    pub project_name: Option<String>,
}

/// 分页查询结果
pub struct TaskListResult {
    pub tasks: Vec<TaskMetadata>,
    pub total: usize,
    pub page: usize,
    pub size: usize,
}

/// 任务数据库记录（含 log_path）
pub struct TaskRecord {
    pub metadata: TaskMetadata,
    pub log_path: String,
}

pub struct TaskDb {
    conn: Connection,
}

impl TaskDb {
    /// 打开数据库，自动建表
    pub fn open() -> Result<Self> {
        let db_path = format!("{}/{}", HOST_LOG_DIR, TASK_DB_FILE);
        let conn = Connection::open(&db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                task_id      TEXT PRIMARY KEY,
                space_name   TEXT NOT NULL,
                project_name TEXT NOT NULL,
                status       TEXT NOT NULL,
                started_at   TEXT NOT NULL,
                finished_at  TEXT,
                duration_ms  INTEGER,
                steps        TEXT NOT NULL DEFAULT '[]',
                log_path     TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_tasks_space ON tasks(space_name);
            CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(space_name, project_name);
            CREATE INDEX IF NOT EXISTS idx_tasks_started_at ON tasks(started_at);",
        )?;

        Ok(Self { conn })
    }

    /// 插入新任务
    pub fn insert_task(&self, metadata: &TaskMetadata, log_path: &str) -> Result<()> {
        let steps_json = serde_json::to_string(&metadata.steps)?;
        self.conn.execute(
            "INSERT INTO tasks (task_id, space_name, project_name, status, started_at, \
             finished_at, duration_ms, steps, log_path) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, \
             ?8, ?9)",
            params![
                metadata.task_id,
                metadata.space_name,
                metadata.project_name,
                metadata.status.to_string(),
                metadata.started_at,
                metadata.finished_at,
                metadata.duration_ms,
                steps_json,
                log_path,
            ],
        )?;
        Ok(())
    }

    /// 更新任务状态
    pub fn update_task(&self, metadata: &TaskMetadata) -> Result<()> {
        let steps_json = serde_json::to_string(&metadata.steps)?;
        self.conn.execute(
            "UPDATE tasks SET status = ?1, finished_at = ?2, duration_ms = ?3, steps = ?4 \
             WHERE task_id = ?5",
            params![
                metadata.status.to_string(),
                metadata.finished_at,
                metadata.duration_ms,
                steps_json,
                metadata.task_id,
            ],
        )?;
        Ok(())
    }

    /// 查询单个任务
    pub fn get_task(&self, task_id: &str) -> Result<Option<TaskRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT task_id, space_name, project_name, status, started_at, finished_at, \
             duration_ms, steps, log_path FROM tasks WHERE task_id = ?1",
        )?;

        let mut rows = stmt.query(params![task_id])?;

        if let Some(row) = rows.next()? {
            let record = Self::row_to_record(row)?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    /// 分页查询任务列表
    pub fn list_tasks(
        &self,
        filter: &TaskFilter,
        page_params: &PageParams,
    ) -> Result<TaskListResult> {
        let (where_clause, where_params) = Self::build_where_clause(filter);

        // 查总数
        let count_sql = format!("SELECT COUNT(*) FROM tasks{}", where_clause);
        let total: usize =
            self.conn.query_row(&count_sql, rusqlite::params_from_iter(&where_params), |row| {
                row.get(0)
            })?;

        // 分页查询
        let offset = (page_params.page.saturating_sub(1)) * page_params.size;
        let query_sql = format!(
            "SELECT task_id, space_name, project_name, status, started_at, finished_at, \
             duration_ms, steps, log_path FROM tasks{} ORDER BY started_at DESC LIMIT ?{} \
             OFFSET ?{}",
            where_clause,
            where_params.len() + 1,
            where_params.len() + 2,
        );

        let mut all_params: Vec<Box<dyn rusqlite::types::ToSql>> = where_params
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn rusqlite::types::ToSql>)
            .collect();
        all_params.push(Box::new(page_params.size as i64));
        all_params.push(Box::new(offset as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            all_params.iter().map(|p| p.as_ref()).collect();

        let mut stmt = self.conn.prepare(&query_sql)?;
        let rows =
            stmt.query_map(param_refs.as_slice(), |row| Ok(Self::row_to_record(row).unwrap()))?;

        let mut tasks = Vec::new();
        for row in rows {
            let record = row?;
            tasks.push(record.metadata);
        }

        Ok(TaskListResult { tasks, total, page: page_params.page, size: page_params.size })
    }

    /// 交互模式用，支持过滤的任务条目列表，返回 (label, task_id)
    pub fn collect_entries_filtered(&self, filter: &TaskFilter) -> Result<Vec<(String, String)>> {
        let (where_clause, where_params) = Self::build_where_clause(filter);

        let query_sql = format!(
            "SELECT task_id, space_name, project_name, status, started_at, duration_ms, \
             steps FROM tasks{} ORDER BY started_at DESC",
            where_clause
        );

        let param_refs: Vec<Box<dyn rusqlite::types::ToSql>> = where_params
            .into_iter()
            .map(|s| Box::new(s) as Box<dyn rusqlite::types::ToSql>)
            .collect();
        let params: Vec<&dyn rusqlite::types::ToSql> =
            param_refs.iter().map(|p| p.as_ref()).collect();

        let mut stmt = self.conn.prepare(&query_sql)?;
        let rows = stmt.query_map(params.as_slice(), |row| {
            let task_id: String = row.get(0)?;
            let space_name: String = row.get(1)?;
            let project_name: String = row.get(2)?;
            let status: String = row.get(3)?;
            let started_at: String = row.get(4)?;
            let duration_ms: Option<i64> = row.get(5)?;
            let steps_json: String = row.get(6)?;
            Ok((task_id, space_name, project_name, status, started_at, duration_ms, steps_json))
        })?;

        let mut result = Vec::new();
        for row in rows {
            let (task_id, space_name, project_name, status, started_at, duration_ms, steps_json) =
                row?;
            let duration_str = match duration_ms {
                Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
                None => "-".to_string(),
            };
            let step_count: usize = serde_json::from_str::<Vec<serde_json::Value>>(&steps_json)
                .map(|v| v.len())
                .unwrap_or(0);

            let label = format!(
                "{}/{}  {}  [{}]  {}  {} ({} steps)",
                space_name, project_name, started_at, status, task_id, duration_str, step_count
            );
            result.push((label, task_id));
        }

        Ok(result)
    }

    /// 删除单个任务
    pub fn delete_task(&self, task_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE task_id = ?1", params![task_id])?;
        Ok(())
    }

    /// 删除指定 space 的所有任务
    pub fn delete_by_space(&self, space_name: &str) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE space_name = ?1", params![space_name])?;
        Ok(())
    }

    /// 删除指定 space/project 的所有任务
    pub fn delete_by_project(&self, space_name: &str, project_name: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM tasks WHERE space_name = ?1 AND project_name = ?2",
            params![space_name, project_name],
        )?;
        Ok(())
    }

    /// 删除所有任务
    pub fn delete_all(&self) -> Result<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        Ok(())
    }

    /// 获取指定 space 所有任务的 log_path
    pub fn list_all_by_space(&self, space_name: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT log_path FROM tasks WHERE space_name = ?1")?;
        let rows = stmt.query_map(params![space_name], |row| row.get(0))?;
        let mut paths = Vec::new();
        for row in rows {
            paths.push(row?);
        }
        Ok(paths)
    }

    /// 获取指定 space/project 所有任务的 log_path
    pub fn list_all_by_project(&self, space_name: &str, project_name: &str) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT log_path FROM tasks WHERE space_name = ?1 AND project_name = ?2")?;
        let rows = stmt.query_map(params![space_name, project_name], |row| row.get(0))?;
        let mut paths = Vec::new();
        for row in rows {
            paths.push(row?);
        }
        Ok(paths)
    }

    fn build_where_clause(filter: &TaskFilter) -> (String, Vec<String>) {
        let mut conditions = Vec::new();
        let mut params = Vec::new();

        if let Some(space) = &filter.space_name {
            params.push(space.clone());
            conditions.push(format!("space_name = ?{}", params.len()));
        }
        if let Some(project) = &filter.project_name {
            params.push(project.clone());
            conditions.push(format!("project_name = ?{}", params.len()));
        }

        let clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        (clause, params)
    }

    fn row_to_record(row: &rusqlite::Row) -> Result<TaskRecord> {
        let task_id: String = row.get(0)?;
        let space_name: String = row.get(1)?;
        let project_name: String = row.get(2)?;
        let status_str: String = row.get(3)?;
        let started_at: String = row.get(4)?;
        let finished_at: Option<String> = row.get(5)?;
        let duration_ms: Option<i64> = row.get(6)?;
        let steps_json: String = row.get(7)?;
        let log_path: String = row.get(8)?;

        let status: TaskStatus = status_str.parse().unwrap();
        let steps = serde_json::from_str(&steps_json).unwrap_or_default();

        Ok(TaskRecord {
            metadata: TaskMetadata {
                task_id,
                space_name,
                project_name,
                status,
                started_at,
                finished_at,
                duration_ms: duration_ms.map(|v| v as u64),
                steps,
            },
            log_path,
        })
    }
}
