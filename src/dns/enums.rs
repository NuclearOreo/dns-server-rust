#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Reserved(u8),
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Query
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Query,
            1 => Opcode::IQuery,
            2 => Opcode::Status,
            _ => Opcode::Reserved(value),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Query => 0,
            Opcode::IQuery => 1,
            Opcode::Status => 2,
            Opcode::Reserved(value) => value,
        }
    }
}
