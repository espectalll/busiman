use std::clone::Clone;
use std::net::IpAddr;

use regex::Regex;

use rocket::request::FromFormValue;

#[derive(Copy, Debug)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn into_slice(&self) -> [u8; 6] {
        self.0
    }
}

impl Clone for MacAddr {
    fn clone(&self) -> MacAddr {
        *self
    }
}

impl<'v> FromFormValue<'v> for MacAddr {
    type Error = &'v str;

    fn from_form_value(form_value: &'v str) -> Result<MacAddr, &'v str> {
        match String::from_form_value(form_value) {
            Ok(addr_str) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
                                                    .unwrap();
                    static ref DIV: Regex = Regex::new(r"[:-]").unwrap();
                }
                if RE.is_match(addr_str.as_str()) {
                    let mut addr: [u8; 6] = [0, 0, 0, 0, 0, 0];
                    let mut i = 0;
                    for s in DIV.split(addr_str.as_str()) {
                        let x = u8::from_str_radix(s, 16).unwrap();
                        addr[i] = x;
                        i += 1;
                    }
                    return Ok(MacAddr(addr));
                } else {
                    Err(form_value)
                }
            }
            _ => Err(form_value),
        }
    }
}

// users
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub password: String,
}

// sessions
#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
}

// companies
#[derive(Queryable)]
pub struct Company {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub ip: IpAddr,
}

// company_devices
#[derive(Queryable)]
pub struct CompanyDevice {
    pub id: i32,
    pub company_id: i32,
    pub name: String,
    pub mac: MacAddr,
}

/*
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("Error: DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error: unable to connect to {}", database_url));
}
*/
