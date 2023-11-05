#[derive(Debug, PartialEq)]
pub enum ErrorType {
    ParserError,
    InvalidMethod,
    InvalidCode,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub err_type: ErrorType,
    pub msg: String,
}
