use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use super::models::*;
use super::schema::*;
use diesel::result::Error;

pub fn create_connection() -> PgConnection {
    dotenv().ok();
    // 環境変数のDATABASE_URLから、接続情報を取得する
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    
}

pub fn insert_user(
    conn:     &PgConnection, 
    name:     &str, 
    service_name: &str)         -> Result<User, Error> {

let new_user = NewUser { name, url_name };
diesel::insert_into(users::table)
    .values(&new_user)
    .get_result(conn)
}

pub fn insert_exam(
    conn:      &PgConnection, 
    name:      &str, 
    description:  &str)        -> Result<Exam, Error> {

let new_exam = NewExam { name, url_name, image_url };
diesel::insert_into(exams::table)
    .values(&new_exam)
    .get_result(conn)
}

pub fn insert_point(
    conn:            &PgConnection, 
    users_id:     i64, 
    exams_id: i64, 
    entrance_at:      chrono::NaiveDateTime) -> Result<Point, Error> {

let new_point = NewPoint { users_id, exams_id, entrance_at };
diesel::insert_into(points::table)
    .values(&new_point)
    .get_result(conn)
}