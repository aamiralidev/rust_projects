use std::fmt::{Formatter, Display, Result as fmtResult};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::NotFound => "Not Found",
            Self::BadRequest => "Bad Request",
        }
    }
}

impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{}", *self as u16)
    }
}
