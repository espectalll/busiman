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

extern crate rand;
use rand::os::OsRng;
use rand::{Rng, thread_rng};
use rand::distributions::Range;

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
    let sid = cookies.find("sid").expect("No cookie!")
                        .value().parse::<i32>().expect("Invalid cookie!");
    let connection = establish_connection();

    match sessions::table.find(sid).load::<Session>(&connection) {
        Ok(session) => {
            // TODO: detect if username matches session UID
            if session.len() == 1 {
                return Ok(NamedFile::open(Path::new("panel/index.html")).ok().unwrap());
            } else if session.len() < 1 {
                return Err(Status::Unauthorized);
            } else {
                return Err(Status::InternalServerError);
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Debug, FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<login_form>")]
fn login(cookies: &Cookies, login_form: Form<LoginForm>) -> Redirect {
    let login = login_form.get();
    let username = &login.username;
    let password = &login.password;
    let connection = establish_connection();

    match cookies.find("sid") {
    	Some(_) => return Redirect::to("/panel"),
    	None => {
    	    match users::table.filter(users::username.eq(username)).load::<User>(&connection) {
    	        Ok(user) => {
    	            if user.len() == 1 && password.clone() == user[0].password {
    	                let mut rnd = OsRng::new().expect("Whoopsie! No seed!");
                        let s = NewSession{
                            id: rnd.gen_range((2 as i32).pow(16), (2 as i32).pow(22)),
                            user_id: user[0].id,
                        };
                        diesel::insert(&s).into(sessions::table)
                            .get_result::<Session>(&connection).expect("Error: Unable to create session!");
                        cookies.add(Cookie::new("sid", s.id.to_string()));
                        return Redirect::to("/panel");
                    } else {
                        return Redirect::to("/");
                    }
                },
                _ => return Redirect::to("/"),
            }
        },
    }
}

#[derive(Debug, FromForm)]
struct NewCompanyReq {
    name: String,
    ip: String,
    user_id: i32,
}

// TODO: Implement decent company creation
#[post("/new_company", data = "<new_form>")]
fn new_company(new_form: Form<NewCompanyReq>) -> JSON<&'static str> {
    let company_req = new_form.get();
    let name = company_req.name.clone();
    let ip = company_req.ip.clone();
    let uid = company_req.user_id;
    let connection = establish_connection();

    diesel::insert(&NewCompany{ name: name, ip: ip, user_id: uid }).into(companies::table)
                       .get_result::<Company>(&connection).expect("Error: Unable to create company!");

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

// TODO: Implement decent WeMos creation
#[post("/new_wemos", data = "<new_form>")]
fn new_wemos(new_form: Form<NewWemosReq>) -> JSON<&'static str> {
    let wemos_req = new_form.get();
    let company_id = wemos_req.company_id;
    let local_ip = &wemos_req.local_ip;
    let device_1_name = &wemos_req.device_1;
    let device_2_name = &wemos_req.device_2;
    let connection = establish_connection();

    let dev1 = diesel::insert(&NewDevice{ name: device_1_name.clone(), status: false }).into(devices::table)
                        .get_result::<Device>(&connection).expect("Error: Unable to create device!");
    let dev2 = diesel::insert(&NewDevice{ name: device_2_name.clone(), status: false }).into(devices::table)
                        .get_result::<Device>(&connection).expect("Error: Unable to create device!");
    diesel::insert(&NewWemos{ company_id: company_id, local_ip: local_ip.clone(), device_1: dev1.id, device_2: dev2.id }).into(wemos::table)
                        .get_result::<Wemos>(&connection).expect("Error: Unable to create WeMos!");

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
    rocket::ignite().mount("/", routes![static_panel, panel, login, new_company, new_wemos, power, static_web, root]).launch();
}
