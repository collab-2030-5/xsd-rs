use roxmltree::Node;

use super::node_parser::parse_node;
use super::types::RsEntity;
use super::xsd_elements::{ElementType, XsdNode};

pub fn parse_complex_content(node: &Node) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in complexContent required");

    parse_node(&content, node)
}
