use std::str;

use crate::lexer::tokens::*;
use crate::lexer::error::*;

use nom::branch::alt;
use nom::combinator::{map, map_res, recognize, opt};
use nom::bytes::complete::{tag, take, take_while1};
use nom::*;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, tuple};
use nom::character::complete::{char, alpha1, alphanumeric1, digit1, multispace0};

macro_rules! syntax {
    ($fn_name: ident, $tag_string: literal, $output_tok: expr) => {
        fn $fn_name<'a>(s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_tok)(s)
        }
    };
}

// operators

syntax! {equal_operator, "==", Token::Equal}
syntax! {not_equal_operator, "!=", Token::NotEqual}
syntax! {exp_operator, "**", Token::Exp}
syntax! {plus_operator, "+", Token::Plus}
syntax! {minus_operator, "-", Token::Minus}
syntax! {mult_operator, "*", Token::Mult}
syntax! {div_operator, "/", Token::Div}
syntax! {not_operator, "!", Token::Not}
syntax! {gte_operator, ">=", Token::GreaterThanEqual}
syntax! {lte_operator, "<=", Token::LessThanEqual}
syntax! {gt_operator, ">", Token::GreaterThan}
syntax! {lt_operator, "<", Token::LessThan}
syntax! {assign_operator, "=", Token::Assign}

fn lex_operator(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        equal_operator,
        not_equal_operator,
        exp_operator,
        plus_operator,
        minus_operator,
        div_operator,
        mult_operator,
        not_operator,
        gte_operator,
        lte_operator,
        gt_operator,
        lt_operator,
        assign_operator,
    ))(input)
}

// punctuation

syntax! {semicolon_punctuation, ";", Token::Semicolon}
syntax! {colon_punctuation, ":", Token::Colon}
syntax! {comma_punctuation, ",", Token::Comma}
syntax! {lparenthesis_punctuation, "(", Token::LParenthesis}
syntax! {rparenthesis_punctuation, ")", Token::RParenthesis}
syntax! {lbrace_punctuation, "{", Token::LBrace}
syntax! {rbrace_punctuation, "}", Token::RBrace}
syntax! {lbracket_punctuation, "[", Token::LBracket}
syntax! {rbracket_punctuation, "]", Token::RBracket}

fn lex_punctuation(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        semicolon_punctuation,
        colon_punctuation,
        comma_punctuation,
        lparenthesis_punctuation,
        rparenthesis_punctuation,
        lbrace_punctuation,
        rbrace_punctuation,
        lbracket_punctuation,
        rbracket_punctuation,
    ))(input)
}

// boolean and logic operations

syntax! {and_boolean_operation, "&", Token::BooleanAnd}
syntax! {or_boolean_operation, "|", Token::BooleanOr}
syntax! {xor_boolean_operation, "^", Token::BooleanXor}
syntax! {lshift_boolean_operation, "<<", Token::LShift}
syntax! {rshift_boolean_operation, ">>", Token::RShift}

syntax! {and_logic_operation, "&&", Token::LogicAnd}
syntax! {or_logic_operation, "||", Token::LogicOr}

fn lex_boolean_operation(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        and_boolean_operation,
        or_boolean_operation,
        xor_boolean_operation,
        lshift_boolean_operation,
        rshift_boolean_operation,
    ))(input)
}

fn lex_logic_operation(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        and_logic_operation,
        or_logic_operation,
    ))(input)
}

// strings

fn non_escaped_char(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|i| -> bool {
        i >= 0x20 && (i != 0x22) && (i != 0x27) && (i != 0x5C)
    })(input)
}


fn escaped_char(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        pair(
            char('\\'), 
            alt((
                char('\''),
                char('"'),
                char('\\'),
            ))
        )
    )(input)
}

fn string_content(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        many0(
            alt((
                non_escaped_char,
                escaped_char
            ))
        )
    )(input)
}

fn convert_slice_to_utf8(s: &[u8]) -> Result<String, ParseError> {
    str::from_utf8(s).map(|s| s.to_owned()).map_err(|e| e.into())
}

fn input_to_string(input: &[u8]) -> IResult<&[u8], String> {
    map_res(delimited(char('"'), string_content, char('"')), |s| convert_slice_to_utf8(s))(input)
}

fn lex_string(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_string, Token::StringLiteral)(input)
}

// chars

fn convert_slice_to_char(s: &[u8]) -> Result<char, ParseError> {
    if s.len() > 4 || s.is_empty() {
        return Err(InvalidCharByteSequenceError::new(s.len()).into())
    }

    let mut buffer: [u8; 4] = [0, 0, 0, 0];
    buffer.clone_from_slice(&s[0..s.len()]);

    let r = u32::from_be_bytes(buffer);
    std::char::from_u32(r).ok_or(CharParseError::new(r).into())
}

fn input_to_char(input: &[u8]) -> IResult<&[u8], char> {
    map_res(delimited(char('\''), string_content, char('\'')), |s| convert_slice_to_char(s))(input)
}

fn lex_char(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_char, Token::CharLiteral)(input)
}

// reserved words and identifiers

fn ident_underscore_prefix(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        tuple((
            tag("_"),
            alphanumeric1,
            many0(alt((alphanumeric1, tag("_")))),
        ))
    )(input)
}

fn ident_alpha_prefix(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        pair(
            alpha1,
            many0(alt((alphanumeric1, tag("_")))),
        )
    )(input)
}

fn lex_ident_or_reserved(input: &[u8]) -> IResult<&[u8], Token> {
    map_res(
        recognize(
            alt((
                ident_underscore_prefix,
                ident_alpha_prefix,
            ))
        ), |i| {
            let s = convert_slice_to_utf8(i);
            s.map(|syntax| match syntax.as_str() {
                "let" => Token::Let,
                "mut" => Token::Mut,
                "fn" => Token::Function,
                "if" => Token::If,
                "elif" => Token::ElseIf,
                "else" => Token::Else,
                "return" => Token::Return,
                "continue" => Token::Continue,
                "break" => Token::Break,
                "true" => Token::BoolLiteral(true),
                "false" => Token::BoolLiteral(false),
                _ => Token::Ident(syntax)
            })
        })(input)
}

// numbers

fn convert_slice_to_number(s: &[u8]) -> Result<i64, ParseError> {
    let r = convert_slice_to_utf8(s)?;
    let i = str::parse::<i64>(r.as_str())?;
    Ok(i)

}

fn input_to_number(input: &[u8]) -> IResult<&[u8], i64> {
    map_res(
        recognize(
            pair(
                opt(char('-')),
                many1(digit1),
            )
        ), |i| {
            convert_slice_to_number(i)
        })(input)
}

fn lex_number(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_number, Token::NumericLiteral)(input)
}

// decimals

fn convert_slice_to_decimal(s: &[u8]) -> Result<f64, ParseError> {
    let r = convert_slice_to_utf8(s)?;
    let f = str::parse::<f64>(r.as_str())?;
    Ok(f)
}

fn input_to_decimal(input: &[u8]) -> IResult<&[u8], f64> {
    map_res(
        recognize(
            tuple((
                pair(opt(char('-')), many1(digit1)),
                char('.'),
                many1(digit1),
            ))
        ), 
        |i| {
            convert_slice_to_decimal(i)
        })(input)
}

fn lex_decimal(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_decimal, Token::DecimalLiteral)(input)
}

// meta

fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

// concrete lexer

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        lex_operator,
        lex_punctuation,
        lex_boolean_operation,
        lex_logic_operation,
        lex_char,
        lex_string,
        lex_ident_or_reserved,
        lex_decimal,
        lex_number,
        lex_illegal,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lexer_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}