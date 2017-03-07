use std::clone::Clone;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::io::prelude::*;

use regex::Regex;

use std::env;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use rocket::request::FromFormValue;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Error: DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error: unable to connect to {}", database_url))
}

#[derive(Copy, Debug)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn new(a: [u8; 6]) -> MacAddr {
        MacAddr { 0: a }
    }
    pub fn into_slice(&self) -> [u8; 6] {
        self.0
    }
    pub fn into_string(&self) -> String {
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3],
                self.0[4],
                self.0[5])
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

#[derive(Copy, Debug)]
pub struct PowerMsg([u8; 2]);

impl PowerMsg {
    pub fn new(a: [u8; 2]) -> Option<PowerMsg> {
        if a[0] < 2 && a[1] < 2 {
            return Some(PowerMsg { 0: a })
        }
        else {
            return None
        }
    }
    pub fn into_slice(&self) -> [u8; 2] {
        self.0
    }
    pub fn into_string(&self) -> String {
        format!("{}{}", self.0[0], self.0[1])
    }
}

impl Clone for PowerMsg {
    fn clone(&self) -> PowerMsg {
        *self
    }
}

impl<'v> FromFormValue<'v> for PowerMsg {
    type Error = &'v str;

    fn from_form_value(form_value: &'v str) -> Result<PowerMsg, &'v str> {
        match String::from_form_value(form_value) {
            Ok(msg_str) => {
                let (a_str, b_str) = msg_str.split_at(1);
                match a_str.parse::<u8>() {
                    Ok(a) => {
                        match b_str.parse::<u8>() {
                            Ok(b) => return Ok(PowerMsg { 0: [a, b] }),
                            _ => return Err(form_value),
                        }
                    },
                    _ => return Err(form_value),
                }
            },
            _ => Err(form_value),
        }
    }
}

pub fn power_req(ip: IpAddr, port: u16, msg: PowerMsg) -> bool {
    match UdpSocket::bind("127.0.0.1:1234") {
        Ok(socket) => {
            let buf = msg.into_slice();
            match socket.send_to(&buf, (ip, port)) {
                Ok(_) => return true,
                Err(_) => {
                    println!("Error: unable to send to {}:{}", ip, port);
                    return false;
                }
            }
        }
        Err(_) => {
            println!("Error: unable to bind");
            return false;
        }
    }
}
