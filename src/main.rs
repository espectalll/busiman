#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::net::IpAddr;
use std::env;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod busiman;
use busiman::*;

mod wakeonlan;

extern crate rocket;
use rocket::http::{Cookie, Cookies, ContentType};
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::content::{Content, JSON};

extern crate dotenv;
use dotenv::dotenv;
// infer_schema!("dotenv:DATABASE_URL");

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
use diesel::Connection;
use diesel::pg::PgConnection;

#[get("/panel")]
fn panel(cookies: &Cookies) -> &'static str {
    /* let connection = establish_connection();
    let results = users.filter(published.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts"); */
    return "Cookies! Yummy!";
}

#[derive(Debug, FromForm)]
struct Device {
    ip: IpAddr,
    port: u16,
    mac_address: MacAddr,
}

#[post("/turnon", data = "<device_form>")]
fn turnon(device_form: Form<Device>) -> JSON<&'static str> {
    let device = device_form.get();
    let ip = device.ip;
    let port = device.port;
    let mac = device.mac_address;

    match wakeonlan::wake_up(ip, port, mac) {
        true => JSON("{ 'success': 'true' }"),
        false => JSON("{ 'success': 'false' }"),
    }
}

#[get("/<path..>")]
fn static_web(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/")]
fn root() -> Option<NamedFile> {
    static_web(PathBuf::from("index.html"))
}

fn main() {
    dotenv().ok();
    rocket::ignite().mount("/", routes![panel, turnon, static_web, root]).launch();
}
