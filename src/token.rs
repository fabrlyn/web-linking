use nom::{bytes::complete::take_while1, combinator::map, IResult};

const OTHER_CHARACTERS: [char; 15] = [
    '!', '#', '$', '%', '&', '\'', '*', '+', '-', '.', '^', '_', '`', '|', '~',
];

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a>(&'a str);

impl<'a> Token<'a> {
    pub fn parse(input: &'a str) -> Option<(&'a str, Token<'a>)> {
        Token::parse_internal(input).ok()
    }

    pub(crate) fn parse_internal(input: &'a str) -> IResult<&'a str, Token<'a>> {
        map(take_while1(is_token_character), Token)(input)
    }
}

fn is_token_character(c: char) -> bool {
    c.is_digit(10) || c.is_ascii_alphabetic() || OTHER_CHARACTERS.contains(&c)
}

#[cfg(test)]
mod tests {

    use super::Token;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("title", Some(("", Token("title"))))]
    #[case("token_v2 abc", Some((" abc", Token("token_v2"))))]
    #[case("<title>", None)]
    fn parse(#[case] input: &str, #[case] expected: Option<(&'_ str, Token<'_>)>) {
        assert_eq!(expected, Token::parse(input));
    }
}
