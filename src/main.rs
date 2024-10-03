// External crates
use env_logger;
use log;

// Internal modules
mod dns;
mod process;

use std::net::UdpSocket;

// Constants
const IP: &str = "127.0.0.1";
const PORT: u16 = 2053;

fn main() {
    env_logger::init();
    log::info!("Starting DNS server");

    let udp_socket = UdpSocket::bind(format!("{IP}:{PORT}")).expect("Failed to bind to address");
    log::info!("Listening on {IP}:{PORT}");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                log::info!("Received {} bytes from {}", size, source);
                let response = process::process(&buf);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                log::error!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
