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
