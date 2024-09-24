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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl Default for RCode {
    fn default() -> Self {
        RCode::NoError
    }
}

impl From<u8> for RCode {
    fn from(value: u8) -> Self {
        match value {
            0 => RCode::NoError,
            1 => RCode::FormatError,
            2 => RCode::ServerFailure,
            3 => RCode::NameError,
            4 => RCode::NotImplemented,
            5 => RCode::Refused,
            _ => RCode::NotImplemented,
        }
    }
}

impl From<RCode> for u8 {
    fn from(r_code: RCode) -> Self {
        match r_code {
            RCode::NoError => 0,
            RCode::FormatError => 1,
            RCode::ServerFailure => 2,
            RCode::NameError => 3,
            RCode::NotImplemented => 4,
            RCode::Refused => 5,
        }
    }
}
