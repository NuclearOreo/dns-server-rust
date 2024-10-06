use crate::dns::enums::{QueryClass, QueryType};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub tokens: Vec<String>,
    pub types: QueryType,
    pub class: QueryClass,
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
        buf.extend_from_slice(&u16::from(self.types).to_be_bytes());
        buf.extend_from_slice(&u16::from(self.class).to_be_bytes());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.length.to_be_bytes());
        buf.extend_from_slice(&self.data);
        buf
    }

    pub fn from_bytes(bytes: &[u8], mut offset: usize) -> (Self, usize) {
        let mut tokens = Vec::new();
        while bytes[offset] != 0 {
            let len = bytes[offset] as usize;
            offset += 1;
            let token = String::from_utf8(bytes[offset..offset + len].to_vec()).unwrap();
            tokens.push(token);
            offset += len;
        }
        offset += 1; // skip the 0 byte
        let types = QueryType::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
        let class = QueryClass::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;
        let ttl = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;
        let length = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let data = bytes[offset..offset + (length as usize)].to_vec();
        offset += length as usize;
        (
            Self {
                tokens,
                types,
                class,
                ttl,
                length,
                data,
            },
            offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_into_bytes() {
        let answer = Answer {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
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

    #[test]
    fn test_answer_from_bytes() {
        let original_answer = Answer {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
            ttl: 3600,
            length: 4,
            data: vec![8, 8, 8, 8],
        };

        let bytes = original_answer.into_bytes();
        let (final_answer, _) = Answer::from_bytes(&bytes, 0);

        assert_eq!(original_answer, final_answer);
    }
}
