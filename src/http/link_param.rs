use nom::IResult;

use crate::{token::Token, Link};

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
        
    }
}
