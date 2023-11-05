#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    /// Create a new http header from a key-value pair.
    pub fn new(key: &str, value: &str) -> Self {
        Header {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl ToString for Header {
    /// Convert the `Header` to a valid http plaintext representation.
    fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Header;

    #[test]
    fn create_new_header() {
        let result = Header::new("Content-Type", "text/html");
        let expected = Header {
            key: "Content-Type".to_string(),
            value: "text/html".to_string(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn string_representation() {
        let result = Header::new("Content-Type", "text/html").to_string();
        let expected = "Content-Type: text/html".to_string();

        assert_eq!(result, expected);
    }
}
