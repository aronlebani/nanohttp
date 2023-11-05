use crate::header::Header;
use crate::status::Status;

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    scheme: String,
    version: String,
    status: Status,
    headers: Vec<Header>,
    content: String,
}

impl Response {
    /// Create a new http response with no body.
    pub fn empty() -> Self {
        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content: String::new(),
        }
    }

    // Create a new http response with a given body. Does not set the `Content-Type` or
    // `Content-Length` header.
    pub fn body(content: &str) -> Self {
        Response {
            scheme: "HTTP".to_string(),
            version: "1.1".to_string(),
            status: Status::Ok,
            headers: Vec::new(),
            content: content.to_string(),
        }
    }

    /// Create a new http response with a given body and content type. Sets the `Content-Type`
    /// header to the content type provided, and automatically sets the `Content-Length` header to
    /// the length of the provided content.
    pub fn content(content: &str, content_type: &str) -> Self {
        let content_length = content.len();

        Self::body(content)
            .header(Header::new("Content-Type", content_type))
            .header(Header::new("Content-Length", &content_length.to_string()))
    }

    /// Create a `html` http response. This method is the same as [Response::content], but it
    /// automatically sets the `Content-Type: text/html` header.
    pub fn html(content: &str) -> Self {
        Self::content(content, "text/html")
    }

    /// Create a `json` http response. This method is the same as [Response::content], but it
    /// automatically sets the `Content-Type: application/json` header.
    pub fn json(content: &str) -> Self {
        Self::content(content, "application/json")
    }

    /// Add a cookie to the http response.
    pub fn cookie(self, content: &str) -> Self {
        self.header(Header::new("Set-Cookie", content))
    }

    /// Set the status of the http response.
    pub fn status(self, status: Status) -> Self {
        Response { status, ..self }
    }

    /// Add a header to the http response.
    pub fn header(self, header: Header) -> Self {
        let mut headers = self.headers;
        headers.push(header);

        Response { headers, ..self }
    }
}

impl ToString for Response {
    /// Convert the `Response` to a valid http plaintext response.
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

#[cfg(test)]
mod tests {
    use crate::Header;
    use crate::Response;
    use crate::Status;

    #[test]
    fn empty_response_scheme() {
        let result = Response::empty();

        assert!(result.to_string().contains("HTTP"));
    }

    #[test]
    fn empty_response_version() {
        let result = Response::empty();

        assert!(result.to_string().contains("1.1"));
    }

    #[test]
    fn empty_response_status() {
        let result = Response::empty();

        assert!(result.to_string().contains("200 OK"));
    }

    #[test]
    fn response_content() {
        let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
        let result = Response::content(html, "text/html");

        assert!(result.to_string().contains(html));
    }

    #[test]
    fn response_content_length_header() {
        let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
        let result = Response::content(html, "text/html");

        assert!(result.to_string().contains("Content-Length: 89"));
    }

    #[test]
    fn set_status() {
        let result = Response::empty().status(Status::Forbidden);

        assert!(result.to_string().contains("403 FORBIDDEN"));
    }

    #[test]
    fn set_header() {
        let result = Response::empty().header(Header::new("Access-Control-Allow-Origin", "*"));

        assert!(result
            .to_string()
            .contains("Access-Control-Allow-Origin: *"));
    }

    #[test]
    fn set_header_does_not_override_existing_headers() {
        let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
        let result = Response::content(html, "text/html")
            .header(Header::new("Access-Control-Allow-Origin", "*"));

        assert!(result.to_string().contains("Content-Length: 89"));
    }

    #[test]
    fn response_format() {
        let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
        let result = Response::content(html, "text/html")
            .status(Status::SeeOther)
            .to_string();
        let expected = "HTTP/1.1 303 SEE OTHER\r\nContent-Type: text/html\r\nContent-Length: 89\r\n\r\n<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";

        assert_eq!(result, expected);
    }
}
