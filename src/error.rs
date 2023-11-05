#[derive(Debug)]
pub enum ErrorType {
    ParserError,
    InvalidMethod,
    InvalidCode,
}

#[derive(Debug)]
pub struct Error {
    pub err_type: ErrorType,
    pub msg: String,
}
