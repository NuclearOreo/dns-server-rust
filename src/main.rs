// External crates
use env_logger;
use log;

// Internal modules
mod dns;

use dns::{enums, Answer, DnsMessage, Header, Question};
use std::net::UdpSocket;

// Constants
const IP: &str = "127.0.0.1";
const PORT: u16 = 2053;

fn main() {
    env_logger::init();
    log::info!("Starting DNS server");

    let dns_message = DnsMessage {
        header: Header {
            packet_id: 1234,
            query_or_response: true,
            opcode: enums::Opcode::Query,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: enums::RCode::NoError,
            question_count: 1,
            answer_count: 1,
            authoritative_count: 0,
            additional_count: 0,
        },
        questions: vec![Question {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: 1,
            class: 1,
        }],
        answers: vec![Answer {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: enums::QueryType::A,
            class: enums::QueryClass::IN,
            ttl: 0,
            length: 0,
            data: vec![8, 8, 8, 8],
        }],
    };

    let udp_socket = UdpSocket::bind(format!("{IP}:{PORT}")).expect("Failed to bind to address");
    log::info!("Listening on {IP}:{PORT}");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                log::info!("Received {} bytes from {}", size, source);
                let response = dns_message.into_bytes();
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
