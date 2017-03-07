use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::io::prelude::*;

use busiman::MacAddr;

pub fn wake_up(ip: IpAddr, port: u16, mac: MacAddr) -> bool {
    match UdpSocket::bind("127.0.0.1:1234") {
        Ok(socket) => {
            let buf = mac.into_slice();
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
