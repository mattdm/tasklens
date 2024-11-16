use dioxus::prelude::*;

use turbosql::Turbosql;

use serde::{Deserialize, Serialize};

use comrak::markdown_to_html;

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

/// the public interface
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
    if let Ok(t) = select!(TaskTable "WHERE rowid = ?", id) {
        assert!(id == t.rowid);
        Ok(TaskInfo {
            task_id: id,
            raw: format!("# {t.title}\n\n{t.raw}\n"),
        })
    } else {
        Err(ServerFnError::ServerError(format!("{e:?}")));
    }
}

/// first, process the raw markdown input to html
#[server]
pub async fn cook(task_in: TaskInfo) -> Result<TaskInfo, ServerFnError> {
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

    cooked = markdown_to_html(t.raw, &markdown_options);

    TaskInfo {
        task_id: task_in.task_id,
        raw: task_in.raw,
        html: cooked,
    };
}

/// save a taskinfo struct to a (more complex) tasktable database structure
#[server]
pub async fn update(task_in: TaskInfo) -> Result<TaskInfo, ServerFnError> {
    let (new_title, new_detail) = match t.raw.split_once('\n') {
        Some((first_line, rest)) => (
            first_line
                .trim_start_matches(|c: char| c == '#' || c.is_whitespace())
                .trim_end()
                .to_string(),
            rest.trim_start().to_string(),
        ),
        // If there's no newline, the entire input is the title
        None => (
            t.raw
                .trim_start_matches(|c: char| c == '#' || c.is_whitespace())
                .trim_end()
                .to_string(),
            String::new(),
        ),
    };

    let timestamp = OffsetDateTime.utc_now();

    execute!(
        "UPDATE tasktable SET title = ?, detail = ?, html = ?, updated =? WHERE rowid = ?",
        new_title,
        new_detail,
        task_in.html,
        timestamp,
        task_in.id
    );

    Ok(t)
}
