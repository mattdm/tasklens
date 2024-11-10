#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TaskId {
    pub task_id: Option<i32>,
}

#[derive(PartialEq, Clone)]
pub struct Task {
    task_id: i32,
    title: String,
    details: String,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}
