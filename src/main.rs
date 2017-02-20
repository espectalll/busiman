#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::io;
use std::net::{IpAddr, Ipv4Addr};

mod busiman;
use busiman::MacAddr;

mod wakeonlan;

extern crate rocket;
use rocket::request::Form;
use rocket::response::content::JSON;

// TODO: Develop PostgreSQL database
// TODO: Implement UI templates

#[derive(Debug, FromForm)]
struct Device {
    mac_address: MacAddr
}

#[post("/turnon", data = "<device>")]
fn turnon(device: Form<Device>) -> JSON<&'static str> {
	let company_ip = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
	let mac_address = device.get().mac_address.clone();

	println!("{:?}", device.get());

	match wakeonlan::wake_up(company_ip, mac_address) {
		true => JSON("{ 'success': 'true' }"),
		false => JSON("{ 'success': 'false' }")
	}
}

fn main() {
	rocket::ignite().mount("/", routes![turnon]).launch();
}
