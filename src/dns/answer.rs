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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_into_bytes() {
        let answer = Answer {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: 1,
            class: 1,
            ttl: 3600,
            length: 4,
            data: vec![8, 8, 8, 8],
        };

        let bytes = answer.into_bytes();
        let expected_bytes = vec![
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
