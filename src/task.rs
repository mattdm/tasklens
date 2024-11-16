use dioxus::prelude::*;

use turbosql::Turbosql;

use serde::{Deserialize, Serialize};

use comrak::markdown_to_html;

use time::OffsetDateTime;

/// the underlying database table
#[derive(Turbosql, Serialize, Clone, Deserialize, Debug)]
pub struct TaskTable {
    rowid: Option<i64>,
    pub raw: String,
    //html: String,
    pub updated: String,
    pub priority: f64,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

impl Default for TaskTable {
    fn default() -> Self {
        TaskTable {
            rowid: None,
            raw: String::new(),
            //html: String::new(),
            updated: format!("1000-01-01"),
            priority: 0.0,
        }
    }
}

#[server]
pub async fn read(id: i64) -> Result<TaskTable, ServerFnError> {
    match select!(TaskTable "WHERE rowid = ?", id) {
        Ok(t) => {
            assert!(id == t.rowid);
            Ok(t)
        }
        Err(e) => Err(ServerFnError::ServerError(format!("{e:?}"))),
    }
}

/// first, process the raw markdown input to html
/// FIXME: does this belong here?
#[server]
pub async fn cook(raw: String) -> Result<String, ServerFnError> {
    let mut markdown_options = comrak::Options::default();
    markdown_options.parse.smart = true;
    markdown_options.parse.relaxed_tasklist_matching = true;
    markdown_options.parse.relaxed_autolinks = true;
    markdown_options.render.escape = true;
    markdown_options.render.list_style = comrak::ListStyleType::Star;
    markdown_options.render.escaped_char_spans = true;
    markdown_options.extension.strikethrough = true;
    markdown_options.extension.autolink = true;
    markdown_options.extension.tasklist = true;
    markdown_options.extension.footnotes = true;
    markdown_options.extension.spoiler = true;

    markdown_to_html(t.raw, &markdown_options);
}

/// save a taskinfo struct to a (more complex) tasktable database structure
#[server]
pub async fn update(mut task_in: TaskTable) -> Result<TaskTable, ServerFnError> {
    task_in.updated = OffsetDateTime.utc_now();

    execute!(
        "UPDATE tasktable SET raw = ?, html = ?, updated =? WHERE rowid = ?",
        task_in.raw,
        task_in.html,
        timestamp,
        task_in.id
    );

    Ok(t)
}
