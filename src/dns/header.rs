use crate::dns::enums::{Opcode, RCode};

#[derive(Default, Debug, Clone, Copy)]
pub struct Header {
    pub packet_id: u16,
    pub query_or_response: bool,
    pub opcode: Opcode,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    pub response_code: RCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub authoritative_count: u16,
    pub additional_count: u16,
}

impl Header {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.packet_id.to_be_bytes());

        let mut byte = (self.query_or_response as u8) << 7;
        byte |= u8::from(self.opcode);
        byte |= (self.authoritative_answer as u8) << 2;
        byte |= (self.truncated_message as u8) << 1;
        byte |= self.recursion_desired as u8;
        buf.push(byte);

        let mut byte = (self.recursion_available as u8) << 7;
        byte |= self.reserved << 4;
        byte |= u8::from(self.response_code);
        buf.push(byte);

        buf.extend_from_slice(&self.question_count.to_be_bytes());
        buf.extend_from_slice(&self.answer_count.to_be_bytes());
        buf.extend_from_slice(&self.authoritative_count.to_be_bytes());
        buf.extend_from_slice(&self.additional_count.to_be_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_into_bytes() {
        let header = Header {
            packet_id: 1234,
            query_or_response: true,
            opcode: Opcode::Query,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: RCode::NoError,
            question_count: 0,
            answer_count: 0,
            authoritative_count: 0,
            additional_count: 0,
        };

        let bytes = header.into_bytes();
        let expected_bytes = vec![
            0x04, 0xD2, // packet_id
            0x80, // query_or_response, opcode, authoritative_answer, truncated_message, recursion_desired
            0x00, // recursion_available, reserved, response_code
            0x00, 0x00, // question_count
            0x00, 0x00, // answer_count
            0x00, 0x00, // authoritative_count
            0x00, 0x00, // additional_count
        ];

        assert_eq!(bytes, expected_bytes);
    }
}
