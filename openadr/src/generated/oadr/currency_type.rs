use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyType {
    pub item_description: crate::oadr::CurrencyItemDescriptionType,
    /// ISO enumeration of currency types, such as USD
    pub item_units: crate::clm5_iso42173a::Iso3AlphaCurrencyCodeContentType,
    pub scale_si_scale_code: crate::scale::SiScaleCodeType,
}

impl CurrencyType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        xsd_util::write_string_enumeration(writer, "itemDescription", self.item_description)?;
        xsd_util::write_string_enumeration(writer, "itemUnits", self.item_units)?;
        xsd_util::write_string_enumeration(writer, "scale:siScaleCode", self.scale_si_scale_code)?;
        Ok(())
    }

    pub(crate) fn write_with_name<W>(
        &self,
        writer: &mut EventWriter<W>,
        name: &str,
        top_level: bool,
        write_type: bool,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        let start = if top_level {
            super::add_schema_attr(events::XmlEvent::start_element(name))
        } else {
            events::XmlEvent::start_element(name)
        };
        let start = if write_type {
            start.attr("xsi:type", "currencyType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for CurrencyType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:currencyType", true, false)?;
        Ok(())
    }
}

impl CurrencyType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut item_description: xsd_util::SetOnce<crate::oadr::CurrencyItemDescriptionType> =
            Default::default();
        let mut item_units: xsd_util::SetOnce<
            crate::clm5_iso42173a::Iso3AlphaCurrencyCodeContentType,
        > = Default::default();
        let mut scale_si_scale_code: xsd_util::SetOnce<crate::scale::SiScaleCodeType> =
            Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                _ => {} // ignore unknown attributes
            };
        }

        loop {
            match reader.next()? {
                xml::reader::XmlEvent::EndElement { name } => {
                    if name.local_name.as_str() == parent_tag {
                        // try to construct struct
                        break;
                    } else {
                        // TODO - make this more specific
                        return Err(xsd_api::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement { name, .. } => {
                    match name.local_name.as_str() {
                        "itemDescription" => item_description
                            .set(xsd_util::read_string_enum(reader, "itemDescription")?)?,
                        "itemUnits" => {
                            item_units.set(xsd_util::read_string_enum(reader, "itemUnits")?)?
                        }
                        "siScaleCode" => scale_si_scale_code
                            .set(xsd_util::read_string_enum(reader, "siScaleCode")?)?,
                        name => {
                            return Err(xsd_api::ReadError::UnexpectedToken(
                                xsd_api::ParentToken(parent_tag.to_owned()),
                                xsd_api::ChildToken(name.to_owned()),
                            ))
                        }
                    }
                }
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::EndDocument => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::Characters(_) => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::ProcessingInstruction { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(CurrencyType {
            item_description: item_description.require()?,
            item_units: item_units.require()?,
            scale_si_scale_code: scale_si_scale_code.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "currencyType")?;
        CurrencyType::read(reader, &attr, "currencyType")
    }
}

impl xsd_api::ReadXml for CurrencyType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match CurrencyType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(xsd_api::ErrorWithLocation {
                    err,
                    line: pos.row,
                    col: pos.column,
                })
            }
        }
    }
}
