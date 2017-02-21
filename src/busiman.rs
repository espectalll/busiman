use std::clone::Clone;
use std::fmt::Debug;

use regex::Regex;

use rocket::request::FromFormValue;

#[derive(Copy, Debug)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn new(addr: [u8; 6]) -> Result<Self, &'static str> {
        if addr[0] <= 0xff && addr[1] <= 0xff && addr[2] <= 0xff && addr[3] <= 0xff &&
           addr[4] <= 0xff && addr[5] <= 0xff {
            return Ok(MacAddr(addr));
        } else {
            return Err("Whoopsie!");
        }
    }

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
                    static ref regex: Regex = Regex::new("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
                                                    .unwrap();
                    static ref div: Regex = Regex::new(r"[:-]").unwrap();
                }
                if regex.is_match(addr_str.as_str()) {
                    let mut addr: [u8; 6] = [0, 0, 0, 0, 0, 0];
                    let mut i = 0;
                    for s in div.split(addr_str.as_str()) {
			let x = u8::from_str_radix(s, 16).unwrap();
                        println!("{:x}", x);
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
