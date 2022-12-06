use std::cell::RefCell;

use roxmltree::Node;

use super::node_parser::parse_node;
use super::types::{RsEntity, Struct, StructField};
use super::utils::{enum_to_field, get_documentation, get_parent_name};
use super::xsd_elements::{ElementType, XsdNode};

pub fn parse_sequence(sequence: &Node, parent: &Node) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: name.into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name)),
        ..Default::default()
    })
}

fn elements_to_fields(sequence: &Node, parent_name: &str) -> Vec<StructField> {
    tracing::debug!("parent_name is: {}", &parent_name);

    let mut fields: Vec<StructField> = Default::default();

    for n in sequence
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
    {
        match parse_node(&n, sequence) {
            RsEntity::StructField(sf) => fields.push(sf),
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", parent_name);
                fields.push(enum_to_field(en))
            }
            RsEntity::Struct(x) => {
                // sequence within a sequence, the fields just get flattened
                for field in x.fields.into_inner() {
                    fields.push(field);
                }
            }
            _ => unreachable!("\nError: {:?}\n{:?}", n, parse_node(&n, sequence)),
        }
    }

    fields
}
