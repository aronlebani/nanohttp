use std::str;

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

#[derive(Debug, Clone)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: &str, value: &str) -> Self {
        Header {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

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

#[derive(Debug)]
pub enum Status {
    Ok,
    SeeOther,
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotAllowed,
}

impl Status {
    fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::SeeOther => 303,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::NotAllowed => 405,
            Status::InternalServerError => 500,
        }
    }

    fn message(&self) -> &str {
        match self {
            Status::Ok => "OK",
            Status::SeeOther => "SEE OTHER",
            Status::BadRequest => "BAD REQUEST",
            Status::Unauthorized => "UNAUTHORIZED",
            Status::Forbidden => "FORBIDDEN",
            Status::NotFound => "NOT FOUND",
            Status::NotAllowed => "NOT ALLOWED",
            Status::InternalServerError => "INTERNAL SERVER ERROR",
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.message())
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub scheme: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn from_string(buffer: &str) -> Result<Self, Error> {
        Self::parse(buffer)
    }

    fn parse(buffer: &str) -> Result<Request, Error> {
        let parser_err = Error {
            err_type: ErrorType::ParserError,
            msg: "Invalid request format".to_string(),
        };

        let mut parts = buffer.split("\r\n");

        let start_line = match parts.next() {
            Some(start_line) => start_line,
            None => return Err(parser_err),
        };

        let (method, path, scheme, version) = Self::parse_start_line(start_line)?;

        let headers: Vec<Header> = parts
            .clone()
            .take_while(|x| x.to_owned() != "")
            .flat_map(|x| Self::parse_header(x))
            .collect();

        let body: String = parts.clone().skip_while(|x| x.to_owned() != "").collect();

        Ok(Request {
            method,
            path: path.to_string(),
            scheme: scheme.to_string(),
            version: version.to_string(),
            headers,
            body,
        })
    }

    fn parse_header(line: &str) -> Result<Header, Error> {
        let parser_err = Error {
            err_type: ErrorType::ParserError,
            msg: "Invalid header format".to_string(),
        };

        let mut parts = line.split(": ");

        let key = match parts.next() {
            Some(key) => key,
            None => return Err(parser_err),
        };

        let value = match parts.next() {
            Some(value) => value,
            None => return Err(parser_err),
        };

        Ok(Header::new(key, value))
    }

    fn parse_protocol(line: &str) -> Result<(&str, &str), Error> {
        let parser_err = Error {
            err_type: ErrorType::ParserError,
            msg: "Invalid protocol format".to_string(),
        };

        let mut parts = line.split("/");

        let scheme = match parts.next() {
            Some(scheme) => scheme,
            None => return Err(parser_err),
        };

        let version = match parts.next() {
            Some(version) => version,
            None => return Err(parser_err),
        };

        Ok((scheme, version))
    }

    fn parse_start_line(line: &str) -> Result<(Method, &str, &str, &str), Error> {
        let parser_err = Error {
            err_type: ErrorType::ParserError,
            msg: "Invalid start line format".to_string(),
        };

        let mut parts = line.split(" ");

        let method = match parts.next() {
            Some(method) => Method::new(method)?,
            None => return Err(parser_err),
        };

        let path = match parts.next() {
            Some(path) => path,
            None => return Err(parser_err),
        };

        let protocol = match parts.next() {
            Some(protocol) => protocol,
            None => return Err(parser_err),
        };

        let (scheme, version) = Self::parse_protocol(protocol)?;

        Ok((method, path, scheme, version))
    }
}

#[derive(Debug)]
pub struct Response {
    scheme: String,
    version: String,
    status: Status,
    headers: Vec<Header>,
    content: String,
}

impl Response {
    pub fn empty() -> Self {
        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content: String::new(),
        }
    }

    pub fn html(content: String) -> Self {
        let content_length = content.len();

        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content,
        }
        .header(Header::new("Content-Type", "text/html"))
        .header(Header::new("Content-Length", &content_length.to_string()))
    }

    pub fn json(content: String) -> Self {
        let content_length = content.len();

        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content,
        }
        .header(Header::new("Content-Type", "application/json"))
        .header(Header::new("Content-Length", &content_length.to_string()))
    }

    pub fn status(self, status: Status) -> Self {
        Response {
            status,
            ..self
        }
    }

    pub fn header(self, header: Header) -> Self {
        let mut headers = self.headers;
        headers.push(header);

        Response { headers, ..self }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .fold(String::new(), |a, b| a + &b.to_string() + "\r\n");

        format!(
            "{}/{} {}\r\n{}\r\n{}",
            self.scheme,
            self.version,
            self.status.to_string(),
            headers,
            self.content
        )
    }
}
