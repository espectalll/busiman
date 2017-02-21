use std::net::{IpAddr, UdpSocket};

use busiman::MacAddr;

pub fn wake_up(request_ip: IpAddr, port: u16, mac_address: MacAddr) -> bool {
    let mut socket;
    match UdpSocket::bind("127.0.0.1:1234") {
        Ok(s) => socket = s,
        Err(_) => {
            println!("Error: unable to bind!");
            return false;
        }
    }
    let buf = mac_address.into_slice();
    match socket.send_to(&buf, (request_ip, port)) {
        Ok(_) => return true,
        Err(_) => {
            println!("Error: unable to send!");
            return false;
        }
    }
}
