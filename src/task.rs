use dioxus::prelude::*;

use turbosql::Turbosql;

use serde::{Deserialize, Serialize};

use time::OffsetDateTime;

/// the underlying database table
#[derive(Turbosql, Debug)]
struct TaskTable {
    rowid: Option<i64>,
    title: String,
    detail: String,
    html: String,
    updated: String,
    priority: f64,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

impl Default for TaskTable {
    fn default() -> Self {
        TaskTable {
            rowid: None,
            title: String::new(),
            detail: String::new(),
            html: String::new(),
            updated: format!("1000-01-01"),
            priority: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskInfo {
    pub task_id: i64,
    pub raw: String,
    pub html: String,
}

impl Default for TaskInfo {
    fn default() -> Self {
        TaskInfo {
            task_id: -1,
            raw: String::new(),
            html: String::new(),
        }
    }
}

#[server]
pub async fn read(id: i64) -> Result<TaskInfo, ServerFnError> {

    match select!(TaskTable "WHERE rowid = ?", id) {
        Ok(t) => Ok(TaskInfo {
            task_id: t.rowid,
            raw: t.raw,
            html: t.html,
        }),
        Err(e) => Err(ServerFnError::ServerError(format!("{e:?}"))),
    }

}


// NEXT: this cooks the markdown and saves everything. make it do that.
#[server]
pub async fn update(t: TaskInfo) -> Result<TaskInfo, ServerFnError> {

    match execute!("UPDATE tasktable SET title = ?, detail = ?,   ")


    Ok(t)
}
