#[derive(Default, Debug, Clone)]
pub struct DNSMessage {
    pub header: Header,
    pub questions: Vec<Question>,
}

impl DNSMessage {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = self.header.into_bytes();
        for question in &self.questions {
            buf.extend_from_slice(&question.into_bytes());
        }
        buf
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Header {
    pub packet_id: u16,
    pub query_or_response: bool,
    // todo - create enum for opcode
    pub opcode: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub reserved: u8,
    // todo - create enum for response code
    pub response_code: u8,
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
        byte |= self.opcode << 3;
        byte |= (self.authoritative_answer as u8) << 2;
        byte |= (self.truncated_message as u8) << 1;
        byte |= self.recursion_desired as u8;
        buf.push(byte);

        let mut byte = (self.recursion_available as u8) << 7;
        byte |= self.reserved << 4;
        byte |= self.response_code;
        buf.push(byte);

        buf.extend_from_slice(&self.question_count.to_be_bytes());
        buf.extend_from_slice(&self.answer_count.to_be_bytes());
        buf.extend_from_slice(&self.authoritative_count.to_be_bytes());
        buf.extend_from_slice(&self.additional_count.to_be_bytes());
        buf
    }
}

#[derive(Default, Debug, Clone)]
pub struct Question {
    pub tokens: Vec<String>,
    // todo - create enum for types
    pub types: u16,
    // todo - create enum for class
    pub class: u16,
}

impl Question {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for token in &self.tokens {
            buf.push(token.len() as u8);
            buf.extend_from_slice(token.as_bytes());
        }
        buf.push(0);
        buf.extend_from_slice(&self.types.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());
        buf
    }
}
