use std::str;

use crate::lexer::tokens::*;
use crate::lexer::error::*;

use nom::branch::alt;
use nom::combinator::{map, map_res, recognize, opt};
use nom::bytes::complete::{tag, take, take_while1};
use nom::*;
use nom::multi::many_m_n;
use nom::multi::{many0, many1};
use nom::sequence::preceded;
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
        preceded(
            char('\\'),
            alt((
                char('\''),
                char('"'),
                char('\\'),
            ))
        )
    )(input)
}

fn string_body(input: &[u8]) -> IResult<&[u8], &[u8]> {
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
    map_res(delimited(char('"'), string_body, char('"')), |s| convert_slice_to_utf8(s))(input)
}

fn lex_string(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_string, Token::StringLiteral)(input)
}

// chars

fn convert_slice_to_char(s: &[u8]) -> Result<char, ParseError> {
    if s.len() > 4 || s.is_empty() {
        return Err(InvalidCharByteSequenceError::new(s.len()).into())
    }

    let chars = str::from_utf8(&s[..])?.chars().collect::<Vec<char>>();
    if chars.len() != 1 {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        s[..].iter().enumerate().for_each(|(i, v)| { buffer[i] = *v });
        return Err(CharParseError::new(&buffer).into())
    }

    Ok(chars[0])
}

fn char_body(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        many_m_n(
            1, 
            4, 
            alt((
                non_escaped_char,
                escaped_char,
            )))
    )(input)
}

fn input_to_char(input: &[u8]) -> IResult<&[u8], char> {
    map_res(delimited(tag("'"), char_body, tag("'")), |s| convert_slice_to_char(s))(input)
}

fn lex_char(input: &[u8]) -> IResult<&[u8], Token> {
    map(input_to_char, Token::CharLiteral)(input)
}

// reserved words and identifiers

fn ident_underscore_prefix(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(
        tuple((
            many1(tag("_")),
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
                "while" => Token::While,
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
        lex_decimal,
        lex_number,
        lex_punctuation,
        lex_logic_operation,
        lex_boolean_operation,
        lex_operator,
        lex_char,
        lex_ident_or_reserved,
        lex_string,
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

// tests

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    macro_rules! check_tokens {
        ($func_name: ident, $input: literal, $expected: expr) => {
            #[test]
            fn $func_name() {
                let input = str_to_u8_slice($input);
                let (_, result) = Lexer::lexer_tokens(input).unwrap();
                let expected = $expected;
                assert_eq!(result, expected);
            }
        };
    }

    macro_rules! token_ident {
        ($val: literal) => {
            Token::Ident(String::from($val))
        };
    }

    macro_rules! token_string {
        ($val: literal) => {
            Token::StringLiteral(String::from($val))
        };
    }

    fn str_to_u8_slice(s: &str) -> &[u8] {
        &s.as_bytes()[..]
    }

    // basic tests

    check_tokens! {test_punctuation, "=+(){},;", vec![
        Token::Assign,
        Token::Plus,
        Token::LParenthesis,
        Token::RParenthesis,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
        Token::EOF,
    ]}

    check_tokens! {test_operators, "+ - / * == ** != >= <= > < !", vec![
        Token::Plus,
        Token::Minus,
        Token::Div,
        Token::Mult,
        Token::Equal,
        Token::Exp,
        Token::NotEqual,
        Token::GreaterThanEqual,
        Token::LessThanEqual,
        Token::GreaterThan,
        Token::LessThan,
        Token::Not,
        Token::EOF,
    ]}

    check_tokens! {test_statements, "if else elif while", vec![
        Token::If,
        Token::Else,
        Token::ElseIf,
        Token::While,
        Token::EOF,
    ]}

    check_tokens! {test_reserved_keywords, "fn return break continue let mut", vec![
        Token::Function,
        Token::Return,
        Token::Break,
        Token::Continue,
        Token::Let,
        Token::Mut,
        Token::EOF,
    ]}

    check_tokens! {test_logic_operations, "&& ||", vec![
        Token::LogicAnd,
        Token::LogicOr,
        Token::EOF,
    ]}

    check_tokens! {test_boolean_operations, "& ^ | << >>", vec![
        Token::BooleanAnd,
        Token::BooleanXor,
        Token::BooleanOr,
        Token::LShift,
        Token::RShift,
        Token::EOF,
    ]}

    // TODO: add escaped strings
    check_tokens! {test_strings, 
        r#""foo" "BaR" "bAZ" "Äpfel" "entrée" "I ❤ Coffee" "2\"""#,
        vec![
        token_string! {"foo"},
        token_string! {"BaR"},
        token_string! {"bAZ"},
        token_string! {"Äpfel"},
        token_string! {"entrée"},
        token_string! {"I ❤ Coffee"},
        token_string! {r#"2""#},
        Token::EOF,
    ]}

    check_tokens! {test_char,
        r#"'a' 'b' 'c' '❤' '\''"#,
        vec![
        Token::CharLiteral('a'),
        Token::CharLiteral('b'),
        Token::CharLiteral('c'),
        Token::CharLiteral('❤'),
        Token::CharLiteral('\''),
        Token::EOF,
    ]}

    check_tokens! {test_bool, "true false", vec![
        Token::BoolLiteral(true),
        Token::BoolLiteral(false),
        Token::EOF,
    ]}

    check_tokens! {test_numeric, "123 345 111111 -33 -5", vec![
        Token::NumericLiteral(123),
        Token::NumericLiteral(345),
        Token::NumericLiteral(111111),
        Token::NumericLiteral(-33),
        Token::NumericLiteral(-5),
        Token::EOF,
    ]}

    check_tokens! {test_decimal, "123.345 11.11 1.23 -1.11 -345.543", vec![
        Token::DecimalLiteral(123.345),
        Token::DecimalLiteral(11.11),
        Token::DecimalLiteral(1.23),
        Token::DecimalLiteral(-1.11),
        Token::DecimalLiteral(-345.543),
        Token::EOF,
    ]}

    // TODO: Add more 
    check_tokens! {test_illegal, "\"", vec![
        Token::Illegal,
        Token::EOF,
    ]}

    check_tokens! {test_ident_names, "_test_ foo2bar bar__ __baz _4_fo0 _2foo4baz_", vec![
        token_ident! {"_test_"},
        token_ident! {"foo2bar"},
        token_ident! {"bar__"},
        token_ident! {"__baz"},
        token_ident! {"_4_fo0"},
        token_ident! {"_2foo4baz_"},
        Token::EOF,
    ]}

    // sequence tests

    check_tokens! {test_mixed_numbers, "11 1.34 -4 -2.2 88 4.4 -17 2 1.44", vec![
        Token::NumericLiteral(11),
        Token::DecimalLiteral(1.34),
        Token::NumericLiteral(-4),
        Token::DecimalLiteral(-2.2),
        Token::NumericLiteral(88),
        Token::DecimalLiteral(4.4),
        Token::NumericLiteral(-17),
        Token::NumericLiteral(2),
        Token::DecimalLiteral(1.44),
        Token::EOF,
    ]}

    check_tokens! {test_mixed_logic_boolean_operators, "& && | || & | && |", vec![
        Token::BooleanAnd,
        Token::LogicAnd,
        Token::BooleanOr,
        Token::LogicOr,
        Token::BooleanAnd,
        Token::BooleanOr,
        Token::LogicAnd,
        Token::BooleanOr,
        Token::EOF,
    ]}

}