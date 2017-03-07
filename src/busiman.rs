use std::clone::Clone;
use std::net::IpAddr;

use regex::Regex;

use rocket::request::FromFormValue;

#[derive(Copy, Debug)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn new(a: [u8; 6]) -> MacAddr {
        MacAddr{0: a}
    }
    pub fn into_slice(&self) -> [u8; 6] {
        self.0
    }

    pub fn into_string(&self) -> String {
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
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
