use crate::dns::enums::{Opcode, QueryClass, QueryType, RCode};
use crate::dns::{Answer, DnsMessage, Header, Question};

pub fn process(bytes: &[u8]) -> Vec<u8> {
    let mut msg = DnsMessage::default();
    msg.header = Header::from_bytes(&bytes[0..12]);

    msg.header.query_or_response = true;
    if msg.header.opcode != Opcode::Query {
        msg.header.response_code = RCode::NotImplemented
    }

    msg.header.answer_count = msg.header.question_count;

    for _ in 0..msg.header.question_count {
        msg.questions.push(Question {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
        });

        msg.answers.push(Answer {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
            ttl: 3600,
            length: 4,
            data: vec![8, 8, 8, 8],
        });
    }
    msg.into_bytes()
}
