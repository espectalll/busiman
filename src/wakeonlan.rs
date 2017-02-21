use std::net::{IpAddr, UdpSocket};

use busiman::MacAddr;

pub fn wake_up(request_ip: IpAddr, mac_address: MacAddr) -> bool {
    let mut socket;
    match UdpSocket::bind("127.0.0.1:1234") {
        Ok(s) => socket = s,
        Err(_) => {
            println!("Error: unable to bind!");
            return false;
        }
    }
    let buf = [1, 2, 3];
    match socket.send_to(&buf, "172.19.200.53:8853") {
        Ok(_) => println!("{:?}", mac_address.into_slice()),
        Err(_) => {
            println!("Error: unable to send!");
            return false;
        }
    }
    true
}
