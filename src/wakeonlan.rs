use std::net::{IpAddr, UdpSocket};

pub fn wake_up(
	request_ip: IpAddr,
	mac_address: MacAddr) -> bool {
/*
	If the `wakeonlan` command is available on your system and your inhability
	to overcome laziness is stopping you from doing work, here's a fancy little
	code snippet that can solve all of mankind's problems! :3

	Command::new("wakeonlan")
		.arg()
		.spawn()
		.expect("Whoops!");
	println!("{:x}", mac_address.into_u64());
*/
	true
}
