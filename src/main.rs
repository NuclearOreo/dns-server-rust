mod dns_message;

use dns_message::{DNSMessage, Header, Question};
use std::net::UdpSocket;

const IP: &str = "127.0.0.1";
const PORT: u16 = 2053;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let dns_message = DNSMessage {
        header: Header {
            packet_id: 1234,
            query_or_response: true,
            opcode: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: 0,
            question_count: 1,
            answer_count: 0,
            authoritative_count: 0,
            additional_count: 0,
        },
        questions: vec![Question {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: 1,
            class: 1,
        }],
    };

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind(format!("{IP}:{PORT}")).expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = dns_message.into_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
