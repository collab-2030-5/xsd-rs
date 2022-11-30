use xml::writer::*;
use xml::common::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentValueType {
    pub current_value_type_choice: crate::ei::CurrentValueTypeChoice,

}

impl CurrentValueType {
    fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
