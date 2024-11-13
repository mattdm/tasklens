#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TaskId {
    pub task_id: Option<i32>,
}

#[derive(PartialEq, Clone)]
pub struct Task {
    pub task_id: i32,
    pub title: String,
    pub detail: String,
    pub title_html: String,
    pub detail_html: String,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

pub fn load(id: i32) -> Task {
    // TODO: load from database!

    Task {
        task_id: id,
        title: format!("Task #{id}"),
        detail: "Details, _details_".to_string(),
        title_html: format!("Task #{id}"),
        detail_html: "<p>Details, <em>details</em></p>\n".to_string(),
    }
}
