use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space0,
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::Link;

use super::link_param::LinkParam;

// TODO: Take into consideration relative vs absolute paths and the MUST requirement to resolve relative paths.
pub struct UriReference<'a>(&'a str);

pub struct LinkValue<'a> {
    uri_reference: UriReference<'a>,
    link_params: Vec<LinkParam<'a>>,
}

impl<'a> LinkValue<'a> {
    pub fn parse(input: &'a str) -> Option<(&'a str, LinkValue<'a>)> {
        LinkValue::parse_internal(input).ok()
    }

    pub(crate) fn parse_internal(input: &'a str) -> IResult<&str, LinkValue<'a>> {
        map(
            tuple((uri_reference, many0(link_param))),
            |(uri_reference, link_params)| LinkValue {
                uri_reference,
                link_params,
            },
        )(input)
    }
}

// TODO: Select the correct variant
fn uri_reference(input: &str) -> IResult<&str, UriReference<'_>> {
    map(
        tuple((tag("<"), take_until(">"), tag(">"))),
        |(_, value, _)| UriReference(value),
    )(input)
}

fn link_param(input: &str) -> IResult<&str, LinkParam<'_>> {
    map(
        tuple((ows, tag(";"), ows, LinkParam::parse_internal)),
        |(_, _, _, link_param)| link_param,
    )(input)
}

fn ows(input: &str) -> IResult<&str, &str> {
    space0(input)
}

impl <'a> From<LinkValue<'a>>  for Link<'a> {
    fn from(value: LinkValue<'a>) -> Self {
        // TODO: Investigtate what to do about the 'anchor' parameter.
        // TODO: Investigate what to do with the rel tag, should maybe use the TryFrom trait instead,
        //       This should probably even be TryFrom for a list of Link since multiple rels indicate multiple Links compacted into one http link-value
        //       See: https://datatracker.ietf.org/doc/html/rfc8288#section-3.3
        todo!()
    }
}
