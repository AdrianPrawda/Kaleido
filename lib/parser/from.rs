// LiteralKind

use std::str::FromStr;

use crate::lexer::tokens::Token;

use super::ast::{LiteralKind, AExprKind, BExprKind, IdentifierKind};

impl From<i64> for LiteralKind {
    fn from(value: i64) -> Self {
        LiteralKind::Number(value)
    }
}

impl From<i32> for LiteralKind {
    fn from(value: i32) -> Self {
        LiteralKind::Number(value.into())
    }
}

impl From<f64> for LiteralKind {
    fn from(value: f64) -> Self {
        LiteralKind::Decimal(value)
    }
}

impl From<f32> for LiteralKind {
    fn from(value: f32) -> Self {
        LiteralKind::Decimal(value.into())
    }
}

impl From<String> for LiteralKind {
    fn from(value: String) -> Self {
        LiteralKind::String(value)
    }
}

impl From<&str> for LiteralKind {
    fn from(value: &str) -> Self {
        // Passing an ill-formatted string should be considered a bug
        LiteralKind::String(String::from_str(value).unwrap())
    }
}

impl From<bool> for LiteralKind {
    fn from(value: bool) -> Self {
        LiteralKind::Bool(value)
    }
}

impl From<char> for LiteralKind {
    fn from(value: char) -> Self {
        LiteralKind::Char(value)
    }
}

// AExprKind

impl From<i64> for AExprKind {
    fn from(value: i64) -> Self {
        AExprKind::Int(value)
    }
}

impl From<i32> for AExprKind {
    fn from(value: i32) -> Self {
        AExprKind::Int(value.into())
    }
}

impl From<f64> for AExprKind {
    fn from(value: f64) -> Self {
        AExprKind::Decimal(value)
    }
}

impl From<f32> for AExprKind {
    fn from(value: f32) -> Self {
        AExprKind::Decimal(value.into())
    }
}

// BExprKind

impl From<bool> for BExprKind {
    fn from(value: bool) -> Self {
        match value {
            true => BExprKind::True,
            false => BExprKind::False,
        }
    }
}

// IdentifierKind

impl From<i64> for IdentifierKind {
    fn from(value: i64) -> Self {
        IdentifierKind::Int64(value)
    }
}

impl From<i32> for IdentifierKind {
    fn from(value: i32) -> Self {
        IdentifierKind::Int64(value.into())
    }
}

impl From<f64> for IdentifierKind {
    fn from(value: f64) -> Self {
        IdentifierKind::Float64(value)
    }
}

impl From<f32> for IdentifierKind {
    fn from(value: f32) -> Self {
        IdentifierKind::Float64(value.into())
    }
}

impl From<String> for IdentifierKind {
    fn from(value: String) -> Self {
        IdentifierKind::String(value)
    }
}

impl From<&str> for IdentifierKind {
    fn from(value: &str) -> Self {
        // Passing an ill-formatted string should be considered a bug
        IdentifierKind::String(String::from_str(value).unwrap())
    }
}

impl From<bool> for IdentifierKind {
    fn from(value: bool) -> Self {
        IdentifierKind::Bool(value)
    }
}

impl From<char> for IdentifierKind {
    fn from(value: char) -> Self {
        IdentifierKind::Char(value)
    }
}