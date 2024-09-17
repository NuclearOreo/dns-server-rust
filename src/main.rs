mod dns_message;

use dns_message::Header;
#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let header = Header {
        packet_id: 1234,
        query_or_response: true,
        opcode: 0,
        authoritative_answer: false,
        truncated_message: false,
        recursion_desired: false,
        recursion_available: false,
        reserved: 0,
        response_code: 0,
        question_count: 0,
        answer_count: 0,
        authoritative_count: 0,
        additional_count: 0,
    };

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = header.to_bytes();
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
