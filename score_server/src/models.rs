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
