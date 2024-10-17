// Internal modules
mod dns;
mod process;

// External Packages
use clap::Parser;
use env_logger;
use log;

// Internal Packages
use std::net::UdpSocket;

// Constants
const IP: &str = "127.0.0.1";
const PORT: u16 = 2053;

// CLI Arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Resolver Address
    #[arg(short, long, default_value_t = String::new())]
    resolver: String,
}

fn main() {
    // Setting logging
    env_logger::init();
    log::info!("Starting DNS server");

    // Parsing Commands
    let args = Args::parse();

    // Creating a UDP listener
    let udp_socket = UdpSocket::bind(format!("{IP}:{PORT}")).expect("Failed to bind to address");
    log::info!("Listening on {IP}:{PORT}");

    // Buffer to store incoming data
    let mut buf = [0; 512];

    // Main loop
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                log::info!("Received {} bytes from {}", size, source);

                let response = if !args.resolver.is_empty() {
                    process::forward(&args.resolver, &udp_socket, &buf)
                } else {
                    process::process(&buf)
                };

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
