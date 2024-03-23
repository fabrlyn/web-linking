pub mod http;
pub mod link;
pub mod links;
pub mod target_attribute;
pub mod token;

use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Link<'a> {
    pub value: Value<'a>,
    pub parameters: Vec<Parameter<'a>>,
}

impl<'a> Link<'a> {
    pub fn from_str(s: &'a str) -> Result<Link<'a>, Box<dyn Error>> {
        Self::parse(s)
            .map(|(_, link)| link)
            .map_err(|e| e.to_string().into())
    }

    fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, value) = Value::parse(input)?;
        let (input, parameters) = many0(Parameter::parse)(input)?;

        Ok((input, Self { value, parameters }))
    }
}

#[derive(Debug, PartialEq)]
pub struct Value<'a> {
    pub value: &'a str,
}

impl<'a> Value<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = tag("<")(input)?;
        let (input, value) = take_until(">")(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, Self { value }))
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    key: &'a str,
    value: Option<&'a str>,
}

impl<'a> Parameter<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((Self::parse_key_value, Self::parse_key))(input)
    }

    fn parse_key(input: &'a str) -> IResult<&'a str, Self> {
        let (input, _) = opt(tag(";"))(input)?;
        let (input, key) = alpha1(input)?;
        Ok((input, Self { key, value: None }))
    }

    fn parse_key_value(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = opt(tag(";"))(input)?;
        let (input, key) = alpha1(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, value) = alphanumeric1(input)?;
        let value = Some(value);

        Ok((input, Self { key, value }))
    }
}

fn link<'a>(input: &'a str) -> IResult<&'a str, Link<'a>> {
    let (input, link) = Link::parse(input)?;
    let (input, _) = opt(tuple((tag(","), multispace0)))(input)?;
    Ok((input, link))
}

pub fn links<'a>(input: &'a str) -> Result<Vec<Link<'a>>, Box<dyn Error>> {
    many0(link)(input)
        .map(|(_, links)| links)
        .map_err(|e| e.to_string().into())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{links, Link, Parameter, Value};

    #[rstest]
    fn link() {
        let input = "</a/b/c>";
        let expected = vec![Link {
            value: Value { value: "/a/b/c" },
            parameters: vec![],
        }];
        assert_eq!(expected, links(input).unwrap());
    }

    #[rstest]
    fn link_with_attribute() {
        let input = "</a/b/c>;ct=42";
        let expected = vec![Link {
            value: Value { value: "/a/b/c" },
            parameters: vec![Parameter {
                key: "ct",
                value: Some("42"),
            }],
        }];
        assert_eq!(expected, links(input).unwrap());
    }

    #[rstest]
    fn link_with_attributes() {
        let input = "</a/b/c>;ct=42;obs";
        let expected = vec![Link {
            value: Value { value: "/a/b/c" },
            parameters: vec![
                Parameter {
                    key: "ct",
                    value: Some("42"),
                },
                Parameter {
                    key: "obs",
                    value: None,
                },
            ],
        }];
        assert_eq!(expected, links(input).unwrap());
    }

    #[rstest]
    fn links_with_attributes() {
        let input = "</a/b/c>;ct=42;obs,\n</d/e/f>;ct=41";
        let expected = vec![
            Link {
                value: Value { value: "/a/b/c" },
                parameters: vec![
                    Parameter {
                        key: "ct",
                        value: Some("42"),
                    },
                    Parameter {
                        key: "obs",
                        value: None,
                    },
                ],
            },
            Link {
                value: Value { value: "/d/e/f" },
                parameters: vec![Parameter {
                    key: "ct",
                    value: Some("41"),
                }],
            },
        ];
        assert_eq!(expected, links(input).unwrap());
    }
}
