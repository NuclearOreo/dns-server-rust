use crate::dns::{Answer, Header, Question};

#[derive(Default, Debug, Clone)]
pub struct DnsMessage {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl DnsMessage {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = self.header.into_bytes();
        for question in &self.questions {
            buf.extend_from_slice(&question.into_bytes());
        }
        for answer in &self.answers {
            buf.extend_from_slice(&answer.into_bytes());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::enums;

    #[test]
    fn test_dns_message_into_bytes() {
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
                response_code: 0,
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
                types: 1,
                class: 1,
                ttl: 3600,
                length: 4,
                data: vec![8, 8, 8, 8],
            }],
        };

        let bytes = dns_message.into_bytes();
        let expected_bytes = vec![
            // Header
            0x04, 0xD2, // packet_id
            0x80, // query_or_response, opcode, authoritative_answer, truncated_message, recursion_desired
            0x00, // recursion_available, reserved, response_code
            0x00, 0x01, // question_count
            0x00, 0x01, // answer_count
            0x00, 0x00, // authoritative_count
            0x00, 0x00, // additional_count
            // Question
            12, b'c', b'o', b'd', b'e', b'c', b'r', b'a', b'f', b't', b'e', b'r',
            b's', // "codecrafters"
            2, b'i', b'o', // "io"
            0,    // end of tokens
            0x00, 0x01, // types
            0x00, 0x01, // class
            // Answer
            12, b'c', b'o', b'd', b'e', b'c', b'r', b'a', b'f', b't', b'e', b'r',
            b's', // "codecrafters"
            2, b'i', b'o', // "io"
            0,    // end of tokens
            0x00, 0x01, // types
            0x00, 0x01, // class
            0x00, 0x00, 0x0E, 0x10, // ttl (3600 seconds)
            0x00, 0x04, // length
            8, 8, 8, 8, // data
        ];

        assert_eq!(bytes, expected_bytes);
    }
}
