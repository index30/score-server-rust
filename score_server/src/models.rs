use super::schema::*;

// usersテーブル用
#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub service_name: String
}

// 新たなレコードの追加用
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub service_name: &'a str,
}

// examsテーブル用
#[derive(Queryable)]
pub struct Exam {
    pub id: i64,
    pub name: String,
    pub description: String
}

#[derive(Insertable)]
#[table_name = "exams"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

// pointsテーブル用
#[derive(Queryable)]
pub struct Point {
    pub id: i64,
    pub users_id: i64,
    pub exams_id: i64,
    pub score: i64,
    pub entrance_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "points"]
pub struct NewPoint<'a> {
    pub users_id: &'a i64,
    pub exams_id: &'a i64,
    pub score: &'a i64,
    pub entrance_at: &'a chrono::NaiveDateTime,
}