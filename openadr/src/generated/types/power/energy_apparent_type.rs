use crate::*;
use xml::writer::*;
use xml::common::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyApparentType {

    // --- these fields come from emix:ItemBaseType ---



    // --- these fields come from power:EnergyItemType ---

    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: enums::SiScaleCodeType,


    // --- these fields come from power:EnergyApparentType ---

    pub item_description: String,
    pub item_units: String,
    pub scale_si_scale_code: enums::SiScaleCodeType,

}

impl EnergyApparentType {
    fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {
        write_simple_tag(writer, "itemDescription", self.item_description.as_str())?;
        write_simple_tag(writer, "itemUnits", self.item_units.as_str())?;
