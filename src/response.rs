use crate::header::Header;
use crate::status::Status;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::Response;
    use crate::Status;
    use crate::Header;

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

    const HTML: &str = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";

    #[test]
    fn html_response_content() {
        let result = Response::html(HTML.to_string());

        assert!(result.to_string().contains(HTML));
    }

    #[test]
    fn html_response_content_type_header() {
        let result = Response::html(HTML.to_string());

        assert!(result.to_string().contains("Content-Type: text/html"));
    }

    #[test]
    fn html_response_content_length_header() {
        let result = Response::html(HTML.to_string());

        assert!(result.to_string().contains("Content-Length: 89"));
    }

    const JSON: &str = "{ \"hello\": \"world\" }";

    #[test]
    fn json_response_content() {
        let result = Response::json(JSON.to_string());

        assert!(result.to_string().contains(JSON));
    }

    #[test]
    fn json_response_content_type_header() {
        let result = Response::json(JSON.to_string());

        assert!(result.to_string().contains("Content-Type: application/json"));
    }

    #[test]
    fn json_response_content_length_header() {
        let result = Response::json(JSON.to_string());

        assert!(result.to_string().contains("Content-Length: 20"));
    }

    #[test]
    fn set_status() {
        let result = Response::empty().status(Status::Forbidden);

        assert!(result.to_string().contains("403 FORBIDDEN"));
    }

    #[test]
    fn set_header() {
        let result = Response::empty().header(Header::new("Access-Control-Allow-Origin", "*"));

        assert!(result.to_string().contains("Access-Control-Allow-Origin: *"));
    }

    #[test]
    fn set_header_does_not_override_existing_headers() {
        let result = Response::html(HTML.to_string()).header(Header::new("Access-Control-Allow-Origin", "*"));

        assert!(result.to_string().contains("Content-Type: text/html"));
    }

    #[test]
    fn response_format() {
        let result = Response::html(HTML.to_string()).status(Status::SeeOther).to_string();
        let expected = "HTTP/1.1 303 SEE OTHER\r\nContent-Type: text/html\r\nContent-Length: 89\r\n\r\n<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";

        assert_eq!(result, expected);
    }
}
