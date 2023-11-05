#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::Status;
    
    #[test]
    fn status_code() {
        let result = Status::Ok.code();
        let expected = 200;

        assert_eq!(result, expected);
    }

    #[test]
    fn status_message() {
        let result = Status::InternalServerError.message();
        let expected = "INTERNAL SERVER ERROR";

        assert_eq!(result, expected);
    }

    #[test]
    fn string_representation() {
        let result = Status::NotFound.to_string();
        let expected = "404 NOT FOUND".to_string();

        assert_eq!(result, expected);
    }
}
