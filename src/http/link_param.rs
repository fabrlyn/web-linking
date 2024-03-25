use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};

// TODO: Investigtate what to do about the 'anchor' parameter.

use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct LinkParam<'a> {
    key: Token<'a>,
    value: Option<Token<'a>>,
}

impl<'a> LinkParam<'a> {
    pub fn decode(input: &'a str) -> Option<LinkParam<'a>> {
        LinkParam::parse(input)
            .filter(|(rest, _)| rest.is_empty())
            .map(|(_, link_param)| link_param)
    }

    pub fn parse(input: &'a str) -> Option<(&'a str, LinkParam<'a>)> {
        LinkParam::parse_internal(input).ok()
    }

    pub(crate) fn parse_internal(input: &'a str) -> IResult<&'a str, LinkParam<'a>> {
        map(tuple((key, opt(with_value))), |(key, value)| LinkParam {
            key,
            value,
        })(input)
    }
}

fn bws(input: &str) -> IResult<&str, &str> {
    space0(input)
}

fn key(input: &str) -> IResult<&str, Token<'_>> {
    map(tuple((Token::parse_internal, bws)), |(token, _)| token)(input)
}

fn with_value(input: &str) -> IResult<&str, Token<'_>> {
    map(tuple((tag("="), bws, value)), |(_, _, token)| token)(input)
}

fn value(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        Token::parse_internal,
        delimited(tag("\""), Token::parse_internal, tag("\"")),
    ))(input)
}
