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
