#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct Task {
    title: String,
    details: String,
    order: i32,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}
