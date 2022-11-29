use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, the_rest_of_the_string) =
            get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, the_rest_of_the_string) =
            get_next_word(the_rest_of_the_string).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) =
            get_next_word(the_rest_of_the_string).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocal);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(index) = path.find('?') {
            query_string = Some(QueryString::from(&path[index + 1..]));
            path = &path[..index];
        }
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}
fn get_next_word(buf_req: &str) -> Option<(&str, &str)> {
    for (i, c) in buf_req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&buf_req[..i], &buf_req[i + 1..]));
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocal,
    InvalidMethod,
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid REquest",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocal => "Invalid Protocal",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Error for ParseError {}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}
