use super::constants::attribute;
use super::types::{Import, RsEntity};
use roxmltree::Node;

pub fn parse_import(node: &Node) -> RsEntity {
    RsEntity::Import(Import {
        name: node.attribute(attribute::NAMESPACE).unwrap_or("").into(),
        location: node
            .attribute(attribute::SCHEMA_LOCATION)
            .unwrap_or("")
            .into(),
        comment: None,
    })
}
