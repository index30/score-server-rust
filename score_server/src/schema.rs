// @generated automatically by Diesel CLI.

diesel::table! {
    exams (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
    }
}
