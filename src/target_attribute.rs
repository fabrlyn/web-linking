use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct TargetAttribute<'a> {
    key: Token<'a>,
    value: Token<'a>,
}

impl<'a> TargetAttribute<'a> {
    pub fn from((key, value): (Token<'a>, Token<'a>)) -> TargetAttribute<'a> {
        TargetAttribute { key, value }
    }
}

impl<'a> TargetAttribute<'a> {
    pub(crate) fn parse_internal(input: &'a str) -> IResult<&'a str, TargetAttribute<'a>> {
        map(
            tuple((parse_key, tag("="), parse_value)),
            |(key, _, value)| TargetAttribute::from((key, value)),
        )(input)
    }

    pub fn parse(input: &'a str) -> Option<(&'a str, TargetAttribute<'a>)> {
        Self::parse_internal(input).ok()
    }
}

fn parse_key<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    Token::parse_internal(input)
}

fn parse_value<'a>(input: &'a str) -> IResult<&'a str, Token<'a>> {
    alt((
        Token::parse_internal,
        delimited(char('"'), Token::parse_internal, char('"')),
    ))(input)
}

#[cfg(test)]
mod tests {

    use super::TargetAttribute;
    use crate::token::Token;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        "title*=UTF-8'de'n%c3%a4chstes%20Kapitel abc", 
        Some((
            " abc", 
            TargetAttribute{
                key: Token::parse("title*").unwrap().1, 
                value: Token::parse("UTF-8'de'n%c3%a4chstes%20Kapitel").unwrap().1
            }
        ))
    )]
    #[case(
        "title*=\"UTF-8'de'n%c3%a4chstes%20Kapitel\" abc", 
        Some((
            " abc", 
            TargetAttribute{
                key: Token::parse("title*").unwrap().1, 
                value: Token::parse("UTF-8'de'n%c3%a4chstes%20Kapitel").unwrap().1
            }
        ))
    )]
    fn parse(#[case] input: &str, #[case] expected: Option<(&str, TargetAttribute<'_>)>) {
        assert_eq!(expected, TargetAttribute::parse(input));
    }
}
