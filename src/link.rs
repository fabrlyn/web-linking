use crate::target_attribute::TargetAttribute;

// A link can be viewed as a statement of the form "link context has a
// link relation type resource at link target, which has target
// attributes".

// The ABNF for the field value is:
//
// Link       = #link-value
// link-value = "<" URI-Reference ">" *( OWS ";" OWS link-param )
// link-param = token BWS [ "=" BWS ( token / quoted-string ) ]

pub struct Link<'a> {
    context: Context,
    relation_type: RelationType,
    target: Target,
    target_attributes: Vec<TargetAttribute<'a>>,
}

impl<'a> Link<'a> {
    pub fn from_http_header_format(input: &str) -> Option<Vec<Link<'a>>> {
        None
    }
}

pub struct Iri;

pub struct Uri;

pub enum Context {
    Iri(Iri),
    Uri(Uri),
}

pub struct RelationType(Iri);

pub struct Target;
