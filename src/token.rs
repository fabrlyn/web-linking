// token          = 1*tchar
//
// tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//                / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//                / DIGIT / ALPHA
//                ; any VCHAR, except delimiters

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{char, one_of},
        is_alphabetic,
    },
    error::{self, ErrorKind},
    multi::many1,
    IResult,
};

const TOKEN_CHARACTERS: [char; 15] = [
    '!', '#', '$', '%', '&', '\'', '*', '+', '-', '.', '^', '_', '`', '|', '~',
];

pub struct Token<'a>(&'a str);

impl<'a> Token<'a> {
    pub fn parsee(input: &'a str) -> IResult<&str, Token<'a>> {
        let (a, b) = take_while1(is_token_character)(input)?;
        Ok((a, Token(b)))
    }

    pub fn parse(input: &'a str) -> Option<(&'a str, Token<'a>)> {
        // Figure out why this type can't be inferred.
        take_while1::<_, _, error::Error<&str>>(is_token_character)(input)
            .ok()
            .map(|(rest, token)| (rest, Token(token)))
    }
}

fn is_token_character(c: char) -> bool {
    c.is_digit(10) || c.is_ascii_alphabetic() || TOKEN_CHARACTERS.contains(&c)
}
