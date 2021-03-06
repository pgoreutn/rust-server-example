use chrono::NaiveDateTime;

pub struct NewUser<'a> {
    pub username: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: &'a str,
    pub encrypted_password: &'a str,
    pub phone: Option<&'a str>,
    pub status: i32,
}

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub encrypted_password: String,
    pub phone: Option<String>,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UpdateUser<'a> {
    pub username: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub phone: Option<&'a str>,
}
