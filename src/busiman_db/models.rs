use super::schema::{sessions, companies, wemos, devices};

#[derive(Queryable, FromForm, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub avatar: String,
    pub background: String,
}

#[derive(Queryable, FromForm, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="sessions"]
pub struct NewSession {
    pub user_id: i32,
}

#[derive(Queryable, FromForm, Debug)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="companies"]
pub struct NewCompany {
    pub name: String,
    pub ip: String,
    pub user_id: i32,
}

#[derive(Queryable, FromForm, Debug)]
pub struct Wemos {
    pub id: i32,
    pub company_id: i32,
    pub local_ip: String,
    pub device_1: i32,
    pub device_2: i32,
}

#[derive(Insertable)]
#[table_name="wemos"]
pub struct NewWemos {
    pub company_id: i32,
    pub local_ip: String,
    pub device_1: i32,
    pub device_2: i32,
}

#[derive(Queryable, FromForm, Debug)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub status: bool,
}

#[derive(Insertable)]
#[table_name="devices"]
pub struct NewDevice {
    pub name: String,
    pub status: bool,
}
