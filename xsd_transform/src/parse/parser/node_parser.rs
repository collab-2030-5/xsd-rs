use roxmltree::Node;

use super::any::parse_any;
use super::any_attribute::parse_any_attribute;
use super::attribute::parse_attribute;
use super::attribute_group::parse_attribute_group;
use super::choice::parse_choice;
use super::complex_content::parse_complex_content;
use super::complex_type::parse_complex_type;
use super::element::parse_element;
use super::extension::parse_extension;
use super::import::parse_import;
use super::list::parse_list;
use super::restriction::parse_restriction;
use super::sequence::parse_sequence;
use super::simple_content::parse_simple_content;
use super::simple_type::parse_simple_type;
use super::types::RsEntity;
use super::union::parse_union;
use super::xsd_elements::{ElementType, XsdNode};

pub fn parse_node(node: &Node, parent: &Node) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        Any => parse_any(node),
        AnyAttribute => parse_any_attribute(node),
        Attribute => parse_attribute(node, parent),
        AttributeGroup => parse_attribute_group(node, parent),
        Choice => parse_choice(node),
        ComplexContent => parse_complex_content(node),
        ComplexType => parse_complex_type(node, parent),
        Element => parse_element(node, parent),
        Extension(_) => parse_extension(node, parent),
        Import | Include => parse_import(node),
        List => parse_list(node),
        Restriction(_) => parse_restriction(node, parent),
        Sequence => parse_sequence(node, parent),
        SimpleContent => parse_simple_content(node),
        SimpleType => parse_simple_type(node, parent),
        Union => parse_union(node),

        _ => unreachable!(
            "Unsupported node:\n {:?}\nparent = {:?}\n",
            node,
            node.parent()
        ),
    }
}
