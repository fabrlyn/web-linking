use std::f32::consts::E;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::opt,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    token::{self, Token},
    Link,
};

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
        let a = tuple((
            Token::parse_internal,
            bws,
            opt((tuple(
                tag("="),
                bws,
                alt((
                    Token::parse_internal,
                    delimited(tag("\""), Token::parse_internal, tag("\"")),
                )),
            )),)),
        ))(input)?;
    }
}

fn bws(input: &str) -> IResult<&str, &str> {
    space0(input)
}
