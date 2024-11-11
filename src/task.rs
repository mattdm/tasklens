#![allow(non_snake_case)]
use diesel::prelude::*;
use dioxus::prelude::*;
use std::env;

#[derive(PartialEq, Props, Clone)]
pub struct TaskId {
    pub task_id: Option<i32>,
}

#[derive(PartialEq, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub taskid: i32,
    pub title: String,
    pub detail: String,
    pub titlehtml: String,
    pub detailhtml: String,
    // TODO: urgency, importance, size, enjoyability, due date, computed priority
}

pub fn load(id: i32) -> Task {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    results = tasks::table
        .filter(taskid.eq(id))
        .limit(1)
        .select(Task::as_select())
        .load(connection);

    match results {
        Ok(result) => result,
        _ => Task {
            taskid: id,
            title: String::new(),
            detail: String::new(),
            titlehtml: format!("Task #{id}"),
            detailhtml: String::new(),
        },
    }
}
