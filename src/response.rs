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
