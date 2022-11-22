pub(crate) mod parser;

pub fn parse_xsd(xsd: &str) -> impl std::fmt::Debug + '_ {
    crate::parse::parser::parse(xsd).unwrap()
}
