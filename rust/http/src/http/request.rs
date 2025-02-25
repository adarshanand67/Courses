use super::method::{self, MethodError};
use crate::http::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, FromStr, Utf8Error};

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: method::Method,
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, ParseError> {
        Ok::<Self, ParseError>(Self {
            path: String::from(""),
            query_string: None,
            method: method::Method::GET,
        })
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        let _ = Ok::<Self, ParseError>(Self {
            path: path.to_string(),
            query_string: query_string.map(|qs| qs.to_string()),
            method,
        });

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    request.chars().enumerate().find_map(|(i, c)| {
        if c == ' ' || c == '\r' {
            Some((&request[..i], &request[i + 1..]))
        } else {
            None
        }
    })
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
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
