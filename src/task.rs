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
    pub details: String,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

pub fn load(id: i32) -> Task {
    // TODO: load from database!

    Task {
        task_id: id,
        title: format!("Task #{id}"),
        details: "_Details_".to_string(),
    }
}
