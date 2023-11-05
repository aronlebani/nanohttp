#[derive(Debug)]
pub enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    fn new(from: &str) -> Result<Self, Error> {
        let method_err = Error {
            err_type: ErrorType::InvalidMethod,
            msg: "Invalid or unsupported http method".to_string(),
        };

        match from {
            "HEAD" => Ok(Method::HEAD),
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(method_err),
        }
    }
}
