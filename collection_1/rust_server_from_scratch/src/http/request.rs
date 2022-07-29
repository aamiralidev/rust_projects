use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Debug};
use std::fmt::Result as fmtResult;
use std::str;
use std::str::Utf8Error;
use std::convert::From;
use crate::http::method::MethodError;
use crate::http::QueryString;

#[derive(Debug)]
pub struct Request<'buf>{
    path:&'buf str,
    query_string:Option<QueryString<'buf>>,
    method:Method,
}

impl<'buf> Request<'buf>{
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(req) => {},
        //     Err(e) => Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(requ) => {}
        //     Err(e) => return Err(e),
        // }
        // we can implement From trait for the specific error to convert that error to our
        // custom error as implemented below for utf8error
        // in this way, we don't need to use match statement, we can simply use one line
        // statement as below. it returns result or the specific error and will convert implicityl
        // to the required error
        let req = str::from_utf8(buf)?;

        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocal, _) = get_next_word(req).ok_or(ParseError::InvalidProtocal)?;

        if protocal != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocal);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        return Ok(Self{
            path,
            query_string,
            method
        })

    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&req[..i], &req[i+1..]))
        }
    }
    None
}

pub enum ParseError { InvalidRequest, InvalidEncoding, InvalidProtocal, InvalidMethod, }

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest" ,
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocal => "InvalidProtocal",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {

}