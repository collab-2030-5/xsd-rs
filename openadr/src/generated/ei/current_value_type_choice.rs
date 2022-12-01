use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentValueTypeChoice {
    EiPayloadFloat(crate::ei::PayloadFloatType),
}
