#[derive(Debug)]
pub enum JsonError {
    ParseError(ErrorDetail),
}

#[derive(Debug)]
pub struct ErrorDetail {
    msg: String,
}

impl ErrorDetail {
    pub fn new(msg: String) -> Self {
        ErrorDetail { msg: msg }
    }
}
