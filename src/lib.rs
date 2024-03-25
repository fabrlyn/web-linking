pub mod http;
pub mod link;
pub mod links;
pub mod target_attribute;
pub mod token;

#[derive(Debug, PartialEq)]
pub struct Context<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Target<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Key<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Value<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct TargetAttribute<'a> {
    key: Key<'a>,
    value: Value<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Link<'a> {
    context: Context<'a>,
    target: Target<'a>,
    target_attribute: Vec<TargetAttribute<'a>>,
}
