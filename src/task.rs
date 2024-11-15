use dioxus::prelude::*;

use turbosql::{execute, select, Turbosql};

use serde::{Deserialize, Serialize};

#[derive(Turbosql, Serialize, Deserialize, Default)]
pub struct Task {
    pub rowid: Option<i64>, // TODO: change to task_id when possible. see
    pub title: String,
    pub detail: String,
    pub title_html: String,
    pub detail_html: String,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

#[server]
pub async fn load(id: i64) -> Result<Task, ServerFnError> {
    // TODO: load from database!

    Ok(Task {
        rowid: Some(id),
        title: format!("Task #{id}"),
        detail: "Details, _details_".to_string(),
        title_html: String::new(),
        detail_html: String::new(),
    })
}

#[server]
pub async fn fake(id: i64) -> Result<i64, ServerFnError> {
    // TODO: load from database!

    Ok(1 as i64)
}
