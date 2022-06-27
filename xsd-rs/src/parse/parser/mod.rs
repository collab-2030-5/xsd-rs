mod any;
mod any_attribute;
mod attribute;
mod attribute_group;
mod choice;
mod complex_content;
mod complex_type;
pub mod constants;
mod element;
mod extension;
mod import;
mod list;
mod node_parser;
mod restriction;
pub mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod tests;
pub mod types;
mod union;
mod utils;
pub mod xsd_elements;

use schema::parse_schema;
use types::RsFile;

// FIXME: Actually pass up errors
#[allow(clippy::result_unit_err)]
pub fn parse(text: &str) -> Result<RsFile, ()> {
    let doc = roxmltree::Document::parse(text).expect("Parse document error");

    let schema = doc
        .root()
        .children()
        .filter(|e| e.is_element())
        .last()
        .expect("Schema element is required");

    Ok(parse_schema(&schema))
}
