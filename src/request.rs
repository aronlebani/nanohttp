use crate::error::{Error, ErrorType};
use crate::header::Header;
use crate::method::Method;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub scheme: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    /// Parse a http plaintext request into a `Request` object.
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
            Some(method) => Method::from_string(method)?,
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

#[cfg(test)]
mod tests {
    use crate::Header;
    use crate::Method;
    use crate::Request;

    #[test]
    fn parse_get_request() {
        let req_string =
            "GET / HTTP/1.1\r\nHost: localhost:3333\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n";
        let result = Request::from_string(req_string).unwrap();

        assert_eq!(result.method, Method::GET);
        assert_eq!(result.path, "/");
        assert_eq!(result.scheme, "HTTP");
        assert_eq!(result.version, "1.1");
        assert_eq!(result.headers[0], Header::new("Host", "localhost:3333"));
        assert_eq!(result.headers[1], Header::new("User-Agent", "curl/7.81.0"));
        assert_eq!(result.headers[2], Header::new("Accept", "*/*"));
    }

    #[test]
    fn parse_post_request() {
        let req_string = "POST /hello-world HTTP/1.1\r\nHost: localhost:3333\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\nContent-Type: application/json\r\nContent-Length: 18\r\n\r\n{ \"hello\": \"world\" }";
        let result = Request::from_string(req_string).unwrap();

        assert_eq!(result.method, Method::POST);
        assert_eq!(result.path, "/hello-world");
        assert_eq!(result.scheme, "HTTP");
        assert_eq!(result.version, "1.1");
        assert_eq!(result.headers[0], Header::new("Host", "localhost:3333"));
        assert_eq!(result.headers[1], Header::new("User-Agent", "curl/7.81.0"));
        assert_eq!(result.headers[2], Header::new("Accept", "*/*"));
        assert_eq!(
            result.headers[3],
            Header::new("Content-Type", "application/json")
        );
        assert_eq!(result.headers[4], Header::new("Content-Length", "18"));
        assert_eq!(result.body, "{ \"hello\": \"world\" }");
    }
}
