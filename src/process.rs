use std::net::UdpSocket;

use crate::dns::enums::{Opcode, RCode};
use crate::dns::{Answer, DnsMessage};

pub fn process(bytes: &[u8]) -> Vec<u8> {
    log::debug!("Received {} bytes", bytes.len());
    log::debug!("{:?}", bytes);
    log::info!("Processing DNS request");

    // TODO: Add error handling
    let mut msg = DnsMessage::from_bytes(bytes);

    log::debug!("Parsed message: {:#?}", msg);
    log::info!("Received {} questions", msg.header.question_count);

    msg.header.query_or_response = true;
    if msg.header.opcode != Opcode::Query {
        msg.header.response_code = RCode::NotImplemented
    }

    log::info!("Answering {} questions", msg.header.question_count);

    msg.header.answer_count = msg.header.question_count;
    for i in 0..msg.header.question_count as usize {
        log::info!("Answering question {}", i);
        msg.answers.push(Answer {
            tokens: msg.questions[i].tokens.clone(),
            types: msg.questions[i].types.clone(),
            class: msg.questions[i].class.clone(),
            ttl: 3600,
            length: 4,
            data: vec![8, 8, 8, 8],
        });
    }

    log::debug!("Processed message: {:#?}", msg);

    msg.into_bytes()
}

pub fn forward(address: &str, socket: &UdpSocket, bytes: &[u8]) -> Vec<u8> {
    log::info!("Forwarding DNS request");
    log::debug!("Bytes: {:?}", bytes);
    let msg = DnsMessage::from_bytes(bytes);
    let mut final_response = DnsMessage::default();

    log::info!("Breaking Question Apart");
    // Breaking Questions Apart
    for i in 0..msg.header.question_count as usize {
        // Buffer for response
        let mut buf = [0; 512];

        // Creating new Message
        let mut new_msg = DnsMessage::default();
        new_msg.header = msg.header.clone();
        new_msg.header.question_count = 1;
        new_msg.questions = vec![msg.questions[i].clone()];

        // Sending, Receiving and Parsing Response
        socket
            .send_to(&new_msg.into_bytes(), address)
            .expect("Expected to Send");
        socket.recv(&mut buf).expect("Expected to Receive bytes");
        let response = DnsMessage::from_bytes(&buf);
        log::debug!("{:#?}", response);

        // Updating Final Response
        final_response.header = response.header.clone();
        if response.questions.len() == 1 {
            final_response.questions.push(response.questions[0].clone());
        }
        if response.answers.len() == 1 {
            final_response.answers.push(response.answers[0].clone());
        }
    }

    final_response.header.question_count = final_response.questions.len() as u16;
    final_response.header.answer_count = final_response.answers.len() as u16;

    log::info!("Sending Final Response");
    final_response.into_bytes()
}
