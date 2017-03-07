#![feature(custom_derive)]
#![feature(plugin)]
#![feature(relaxed_adts)] // TODO: see rust-lang/rust#35626
#![plugin(rocket_codegen)]

use std::net::IpAddr;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

extern crate regex;

extern crate dotenv;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
#[macro_use]
extern crate diesel_codegen;

mod busiman_db;
mod busiman;
use busiman_db::models::*;
use busiman_db::schema::*;
use busiman::*;

extern crate rocket;
use rocket::http::{Cookie, Cookies, ContentType, Status};
use rocket::request::Form;
use rocket::response::{Response, Responder, NamedFile, Redirect};
use rocket::response::content::{Content, JSON};


#[get("/panel/<path..>")]
fn static_panel(path: PathBuf, cookies: &Cookies) -> Option<NamedFile> {
    NamedFile::open(Path::new("panel/").join(path)).ok()
}

#[get("/panel")]
fn panel(cookies: &Cookies) -> Result<NamedFile, Status> {
    let session_id = cookies.find("session_id").expect("No cookie!")
                        .value().parse::<i32>().expect("Invalid cookie!");
    let connection = establish_connection();

    match sessions::table.find(session_id).load::<Session>(&connection) {
        Ok(session) => {
            println!("{:?}", session);
            return Ok(NamedFile::open(Path::new("panel/index.html")).ok().unwrap())
        },
        Err(_) => {
            return Err(Status::Unauthorized)
        }
    }
}

#[derive(Debug, FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

// TODO: Send (valid) cookies to user!
#[post("/login", data = "<login_form>")]
fn login(login_form: Form<LoginForm>) -> Redirect {
    let login = login_form.get();
    let username = &login.username;
    let password = &login.password;
    let connection = establish_connection();

    match users::table.filter(users::username.eq(username)).load::<User>(&connection) {
        Ok(user) => {
            println!("User: {:?}", user);
            // println!("Username: {:?}, Password: {:?}", user.username, user.password);
            let s = NewSession{ user_id: 1 };
            let this_session = diesel::insert(&s).into(sessions::table)
                .get_result::<Session>(&connection).expect("Error: Unable to create session!");
            return Redirect::to("/panel");
                // .adjoin_raw_header(format!("Set-Cookie: session_id={}", 1));
        },
        Err(_) => {
            return Redirect::to("/");
        }
    }
}

#[derive(Debug, FromForm)]
struct NewCompanyReq {
    name: String,
    ip: String,
}

// TODO: Implement company creation
#[post("/new_company", data = "<new_form>")]
fn new_company(new_form: Form<NewCompanyReq>) -> JSON<&'static str> {
    let company = new_form.get();
    let name = &company.name;
    let ip = &company.ip;

    return JSON("{ 'success': 'true' }");
}

// TODO: Implement company deletion

#[derive(Debug, FromForm)]
struct NewWemosReq {
    company_id: i32,
    local_ip: String,
    device_1: String,
    device_2: String,
}

// TODO: Implement WeMos creation
#[post("/new_wemos", data = "<new_form>")]
fn new_wemos(new_form: Form<NewWemosReq>) -> JSON<&'static str> {
    let company = new_form.get();
    let local_ip = &company.local_ip;
    let device_1_name = &company.device_1;
    let device_2_name = &company.device_2;

    return JSON("{ 'success': 'true' }");
}

// TODO: Implement WeMos deletion

#[derive(Debug, FromForm)]
struct WemosConnection {
    ip: IpAddr,
    port: u16,
    msg: PowerMsg,
}

#[post("/power", data = "<wemos_form>")]
fn power(wemos_form: Form<WemosConnection>) -> JSON<&'static str> {
    let device = wemos_form.get();
    let ip = device.ip;
    let port = device.port;
    let msg = device.msg;

    match busiman::power_req(ip, port, msg) {
        true => JSON("{ 'success': 'true' }"),
        false => JSON("{ 'success': 'false' }"),
    }
}

// TODO: Needs better handling for directories?
#[get("/<path..>")]
fn static_web(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/")]
fn root() -> Option<NamedFile> {
    static_web(PathBuf::from("index.html"))
}

fn main() {
    // println!("{:?}", users::table::all_columns());
    rocket::ignite().mount("/", routes![static_panel, panel, login, new_company, new_wemos, power, static_web, root]).launch();
}
