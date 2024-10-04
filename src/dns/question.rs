use crate::dns::enums::{QueryClass, QueryType};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub tokens: Vec<String>,
    pub types: QueryType,
    pub class: QueryClass,
}

impl Question {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for token in &self.tokens {
            buf.push(token.len() as u8);
            buf.extend_from_slice(token.as_bytes());
        }
        buf.push(0);
        buf.extend_from_slice(&u16::from(self.types).to_be_bytes());
        buf.extend_from_slice(&u16::from(self.class).to_be_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        let mut tokens = Vec::new();
        let mut offset = 0;

        // Parse tokens
        loop {
            let token_len = bytes[offset] as usize;
            offset += 1;
            let token = String::from_utf8_lossy(&bytes[offset..offset + token_len]).into_owned();
            tokens.push(token);
            offset += token_len;

            if bytes[offset] == 0 {
                offset += 1;
                break;
            }
        }

        // Parse types and class
        let types = QueryType::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;

        let class = QueryClass::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;

        (
            Question {
                tokens,
                types,
                class,
            },
            offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_into_bytes() {
        let question = Question {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
        };

        let bytes = question.into_bytes();
        let expected_bytes = vec![
            12, b'c', b'o', b'd', b'e', b'c', b'r', b'a', b'f', b't', b'e', b'r',
            b's', // "codecrafters"
            2, b'i', b'o', // "io"
            0,    // end of tokens
            0x00, 0x01, // types
            0x00, 0x01, // class
        ];

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_question_from_bytes() {
        let from_bytes = vec![
            12, b'c', b'o', b'd', b'e', b'c', b'r', b'a', b'f', b't', b'e', b'r',
            b's', // "codecrafters"
            2, b'i', b'o', // "io"
            0,    // end of tokens
            0x00, 0x01, // types
            0x00, 0x01, // class
        ];

        let (question, _) = Question::from_bytes(&from_bytes);
        let expected_question = Question {
            tokens: vec!["codecrafters".to_string(), "io".to_string()],
            types: QueryType::A,
            class: QueryClass::IN,
        };

        assert_eq!(question, expected_question);
    }
}
