use std::{error, fmt};

#[derive(Debug)]
pub enum ParseError {
    IntParseError(std::num::ParseIntError),
    FloatParseError(std::num::ParseFloatError),
    StringParseError(std::str::Utf8Error),
    CharParseError(CharParseError),
    InvalidCharByteSequence(InvalidCharByteSequenceError),
}

#[derive(Debug)]
pub struct CharParseError {
    data: u32
}

impl CharParseError {
    pub fn new(c: u32) -> CharParseError {
        CharParseError { data: c }
    }
}

#[derive(Debug)]
pub struct InvalidCharByteSequenceError {
    len_was: usize
}

impl InvalidCharByteSequenceError {
    pub fn new(was: usize) -> InvalidCharByteSequenceError {
        InvalidCharByteSequenceError { len_was: was }
    }
}

// Display impl

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IntParseError(ref err) => err.fmt(f),
            ParseError::FloatParseError(ref err) => err.fmt(f),
            ParseError::StringParseError(ref err) => err.fmt(f),
            ParseError::CharParseError(ref err) => err.fmt(f),
            ParseError::InvalidCharByteSequence(ref err) => err.fmt(f),
        }
    }
}

impl fmt::Display for InvalidCharByteSequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Char sequence should be between 1 and 4 bytes long, but was {}", self.len_was)
    }
}

impl fmt::Display for CharParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Char sequence {:#06x} is not a valid UTF Char", self.data)
    }
}

// error::Error implementation

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            ParseError::IntParseError(ref err) => Some(err),
            ParseError::FloatParseError(ref err) => Some(err),
            ParseError::StringParseError(ref err) => Some(err),
            _ => self.source()
        }
    }
}

impl error::Error for InvalidCharByteSequenceError {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

impl error::Error for CharParseError {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

// From implementation

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
        ParseError::IntParseError(err)
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> ParseError {
        ParseError::FloatParseError(err)
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(err: std::str::Utf8Error) -> ParseError {
        ParseError::StringParseError(err)
    }
}

impl From<InvalidCharByteSequenceError> for ParseError {
    fn from(err: InvalidCharByteSequenceError) -> ParseError {
        ParseError::InvalidCharByteSequence(err)
    }
}

impl From<CharParseError> for ParseError {
    fn from(err: CharParseError) -> ParseError {
        ParseError::CharParseError(err)
    }
}