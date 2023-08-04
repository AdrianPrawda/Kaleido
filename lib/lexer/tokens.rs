use std::ops::{RangeFull, RangeFrom, RangeTo, Range};
use std::iter::Enumerate;

use nom::*;


#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // identifier
    Ident(String),
    StringLiteral(String),
    CharLiteral(char),
    NumericLiteral(i64),
    DecimalLiteral(f64),
    BoolLiteral(bool),

    // operators
    Plus,
    Minus,
    Div,
    Mult,
    Modulo,
    Equal,
    Exp,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    Not,
    Assign,
    FunctionReturn,

    // statements
    If,
    ElseIf,
    Else,
    While,

    // reserved keywords
    Function,
    Return,
    Break,
    Continue,
    Let,
    Mut,

    // logic operations
    LogicAnd,
    LogicOr,

    // boolean operations
    BooleanAnd,
    BooleanXor,
    BooleanOr,
    LShift,
    RShift,

    // punctuations
    Semicolon,
    Colon,
    Comma,
    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

// Tokens implementations

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(init: &'a [Token]) -> Self {
        Tokens { tokens: init, start: 0, end: init.len() }
    }
}

impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens { 
            tokens: &self.tokens[..count], 
            start: 0, 
            end: count 
        }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let first = Tokens {
            tokens: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tokens: suffix,
            start: 0,
            end: suffix.len(),
        };
        (first, second)
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens { 
            tokens: self.tokens.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end, 
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens { tokens: self.tokens, start: self.start, end: self.end }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;

    type Iter = Enumerate<::std::slice::Iter<'a, Token>>;

    type IterElem = ::std::slice::Iter<'a, Token>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
      where
        P: Fn(Self::Item) -> bool {
        self.tokens.iter().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}

// Token implementations

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}