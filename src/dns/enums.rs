#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ALL = 255,
    Reserved(u16) = 0,
}

impl Default for QueryType {
    fn default() -> Self {
        QueryType::A
    }
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            2 => QueryType::NS,
            3 => QueryType::MD,
            4 => QueryType::MF,
            5 => QueryType::CNAME,
            6 => QueryType::SOA,
            7 => QueryType::MB,
            8 => QueryType::MG,
            9 => QueryType::MR,
            10 => QueryType::NULL,
            11 => QueryType::WKS,
            12 => QueryType::PTR,
            13 => QueryType::HINFO,
            14 => QueryType::MINFO,
            15 => QueryType::MX,
            16 => QueryType::TXT,
            28 => QueryType::AAAA,
            33 => QueryType::SRV,
            252 => QueryType::AXFR,
            253 => QueryType::MAILB,
            254 => QueryType::MAILA,
            255 => QueryType::ALL,
            _ => QueryType::Reserved(value),
        }
    }
}

impl From<QueryType> for u16 {
    fn from(query_type: QueryType) -> Self {
        match query_type {
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::MD => 3,
            QueryType::MF => 4,
            QueryType::CNAME => 5,
            QueryType::SOA => 6,
            QueryType::MB => 7,
            QueryType::MG => 8,
            QueryType::MR => 9,
            QueryType::NULL => 10,
            QueryType::WKS => 11,
            QueryType::PTR => 12,
            QueryType::HINFO => 13,
            QueryType::MINFO => 14,
            QueryType::MX => 15,
            QueryType::TXT => 16,
            QueryType::AAAA => 28,
            QueryType::SRV => 33,
            QueryType::AXFR => 252,
            QueryType::MAILB => 253,
            QueryType::MAILA => 254,
            QueryType::ALL => 255,
            QueryType::Reserved(value) => value,
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
    Reserved(u16),
}

impl Default for QueryClass {
    fn default() -> Self {
        QueryClass::IN
    }
}

impl From<u16> for QueryClass {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryClass::IN,
            2 => QueryClass::CS,
            3 => QueryClass::CH,
            4 => QueryClass::HS,
            255 => QueryClass::ANY,
            _ => QueryClass::Reserved(value),
        }
    }
}

impl From<QueryClass> for u16 {
    fn from(query_class: QueryClass) -> Self {
        match query_class {
            QueryClass::IN => 1,
            QueryClass::CS => 2,
            QueryClass::CH => 3,
            QueryClass::HS => 4,
            QueryClass::ANY => 255,
            QueryClass::Reserved(value) => value,
        }
    }
}
