use std::clone::Clone;
use std::fmt::Debug;

use rocket::request::FromFormValue;

#[derive(Copy, Debug)]
pub struct MacAddr(u64);

impl MacAddr {
	pub fn new(addr: u64) -> Result<Self, &'static str> {
		if addr <= 0xff_ff_ff_ff_ff_ff {
			return Ok(MacAddr(addr))
		} else {
			return Err("Whoopsie!");
		}
	}

	pub fn into_u64(&self) -> u64 { self.0 }
}

impl Clone for MacAddr {
	fn clone(&self) -> MacAddr { *self }
}

impl<'v> FromFormValue<'v> for MacAddr {
	type Error = &'v str;

	fn from_form_value(form_value: &'v str) -> Result<MacAddr, &'v str> {
		match u64::from_form_value(form_value) {
			Ok(addr) if addr <= 0xff_ff_ff_ff_ff_ff => Ok(MacAddr(addr)),
			_ => Err(form_value),
		}
	}
}
