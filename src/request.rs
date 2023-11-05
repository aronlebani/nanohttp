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
