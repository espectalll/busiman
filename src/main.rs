#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::io;
use std::net::{IpAddr, Ipv4Addr};

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod busiman;
use busiman::MacAddr;

mod wakeonlan;

extern crate rocket;
use rocket::http::{Cookie, Cookies, ContentType};
use rocket::request::Form;
use rocket::response::content::{Content, JSON};

// TODO: Develop PostgreSQL database
// TODO: Implement UI templates

#[derive(Debug, FromForm)]
struct Device {
    company_ip: IpAddr,
    port: u16,
    mac_address: MacAddr
}

#[get("/")]
fn root(cookies: &Cookies) -> Content<&'static str> {
    Content(ContentType::HTML,
            "<h1>Here shall be nekos :3</h1>
             <h2>2017 @espectalll</h2>")
}

#[post("/turnon", data = "<device_form>")]
fn turnon(device_form: Form<Device>) -> JSON<&'static str> {
    let device = device_form.get();
    let company_ip = device.company_ip;
    let port = device.port;
    let mac_address = device.mac_address;

    match wakeonlan::wake_up(company_ip, port, mac_address) {
        true => JSON("{ 'success': 'true' }"),
        false => JSON("{ 'success': 'false' }"),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![root, turnon]).launch();
}
