use crate::error::{Error, ErrorType};

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Method {
    // Create a new `Method` from a string representation.
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
            "PATCH" => Ok(Method::PATCH),
            _ => Err(method_err),
        }
    }
}

impl ToString for Method {
    // Get the string representation of a `Method`.
    fn to_string(&self) -> String {
        match self {
            Self::HEAD => "HEAD",
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::PATCH => "PATCH",
        }
        .to_string()
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
