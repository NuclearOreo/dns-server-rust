#[derive(Default, Debug, Clone)]
pub struct Answer {
    pub tokens: Vec<String>,
    // todo - create enum for types
    pub types: u16,
    pub class: u16,
    // todo - create enum for types
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl Answer {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for token in &self.tokens {
            buf.push(token.len() as u8);
            buf.extend_from_slice(token.as_bytes());
        }
        buf.push(0);
        buf.extend_from_slice(&self.types.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.length.to_be_bytes());
        buf.extend_from_slice(&self.data);
        buf
    }
}
