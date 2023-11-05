use crate::error::{Error, ErrorType};

#[derive(Debug, PartialEq)]
pub enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    // Create a new `Method` from a string representation. Supported methods are HEAD, GET, POST,
    // PUT, DELETE.
    pub fn from_string(from: &str) -> Result<Self, Error> {
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

#[cfg(test)]
mod tests {
    use crate::Method;

    #[test]
    fn method_from_string() {
        let result = Method::from_string("GET");
        let expected = Ok(Method::GET);

        assert_eq!(result, expected);
    }

    #[test]
    fn method_from_invalid_string() {
        let result = Method::from_string("HELLO");

        assert!(result.is_err());
    }
}
