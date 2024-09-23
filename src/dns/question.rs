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
