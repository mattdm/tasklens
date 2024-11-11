// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (taskid) {
        taskid -> Integer,
        title -> Nullable<Text>,
        detail -> Nullable<Text>,
        titlehtml -> Nullable<Text>,
        titledetail -> Nullable<Text>,
    }
}
