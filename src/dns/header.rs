use crate::dns::enums::{Opcode, RCode};

pub const HEADER_SIZE: usize = 12;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
        byte |= u8::from(self.opcode) << 3;
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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut header = Header::default();

        // packet_id (2 bytes)
        header.packet_id = u16::from_be_bytes([bytes[0], bytes[1]]);

        // flags (2 bytes)
        let flags = u16::from_be_bytes([bytes[2], bytes[3]]);
        header.query_or_response = (flags & (1 << 15)) != 0;
        header.opcode = Opcode::from(((flags >> 11) & 0x0F) as u8);
        header.authoritative_answer = (flags & (1 << 10)) != 0;
        header.truncated_message = (flags & (1 << 9)) != 0;
        header.recursion_desired = (flags & (1 << 8)) != 0;
        header.recursion_available = (flags & (1 << 7)) != 0;
        header.reserved = (flags >> 4) as u8;
        header.response_code = RCode::from((flags & 0x0F) as u8);

        // question_count, answer_count, authoritative_count, additional_count (2 bytes each)
        header.question_count = u16::from_be_bytes([bytes[4], bytes[5]]);
        header.answer_count = u16::from_be_bytes([bytes[6], bytes[7]]);
        header.authoritative_count = u16::from_be_bytes([bytes[8], bytes[9]]);
        header.additional_count = u16::from_be_bytes([bytes[10], bytes[11]]);

        header
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

    #[test]
    fn test_header_from_bytes() {
        let bytes = vec![
            0x04, 0xD2, // packet_id
            0x80, 0x00, // flags
            0x00, 0x01, // question_count
            0x00, 0x01, // answer_count
            0x00, 0x00, // authoritative_count
            0x00, 0x00, // additional_count
        ];

        let expected_header = Header {
            packet_id: 1234,
            query_or_response: true,
            opcode: Opcode::Query,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: false,
            reserved: 0,
            response_code: RCode::NoError,
            question_count: 1,
            answer_count: 1,
            authoritative_count: 0,
            additional_count: 0,
        };

        let header = Header::from_bytes(&bytes);
        assert_eq!(header, expected_header);
    }
}
