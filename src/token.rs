use nom::{bytes::complete::take_while1, combinator::map, IResult};

// RFC mention: https://datatracker.ietf.org/doc/html/rfc8288#section-1.1
// RFC definition: https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6
// ALPHA, DIGIT definition: https://datatracker.ietf.org/doc/html/rfc5234#appendix-B.1

//token          = 1*tchar

//tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//               / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//               / DIGIT / ALPHA
               
// ALPHA          =  %x41-5A / %x61-7A   ; A-Z / a-z

// DIGIT          =  %x30-39 ; 0-9

const OTHER_CHARACTERS: [char; 15] = [
    '!', '#', '$', '%', '&', '\'', '*', '+', '-', '.', '^', '_', '`', '|', '~',
];

#[derive(Clone, Debug)]
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

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
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
