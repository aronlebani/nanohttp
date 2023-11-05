#[derive(Debug, PartialEq, Clone)]
pub enum ErrorType {
    ParserError,
    InvalidMethod,
    InvalidCode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub err_type: ErrorType,
    pub msg: String,
}
