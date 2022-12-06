use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SignatureType {
    pub ds_signed_info: crate::ds::SignedInfoType,
    pub ds_signature_value: crate::ds::SignatureValueType,
    pub ds_key_info: Option<crate::ds::KeyInfoType>,
    pub ds_object: Vec<crate::ds::ObjectType>,
    pub id: Option<String>,
}

impl SignatureType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.ds_signed_info
            .write_with_name(writer, "ds:SignedInfo", false, false)?;
        self.ds_signature_value
            .write_with_name(writer, "ds:SignatureValue", false, false)?;
        if let Some(elem) = &self.ds_key_info {
            elem.write_with_name(writer, "ds:KeyInfo", false, false)?;
        }
        for item in &self.ds_object {
            item.write_with_name(writer, "ds:Object", false, false)?;
        }
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
        // ---- start attributes ----
        let start = match &self.id {
            Some(attr) => start.attr("Id", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "ds:SignatureType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for SignatureType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ds:SignatureType", true, false)?;
        Ok(())
    }
}

impl SignatureType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ds_signed_info: xsd_util::SetOnce<crate::ds::SignedInfoType> = Default::default();
        let mut ds_signature_value: xsd_util::SetOnce<crate::ds::SignatureValueType> =
            Default::default();
        let mut ds_key_info: xsd_util::SetOnce<crate::ds::KeyInfoType> = Default::default();
        let mut ds_object: Vec<crate::ds::ObjectType> = Default::default();
        let mut id: xsd_util::SetOnce<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "Id" => id.set(attr.value.clone())?,
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
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "ds:SignedInfo" => ds_signed_info.set(crate::ds::SignedInfoType::read(
                        reader,
                        &attributes,
                        "ds:SignedInfo",
                    )?)?,
                    "ds:SignatureValue" => {
                        ds_signature_value.set(crate::ds::SignatureValueType::read(
                            reader,
                            &attributes,
                            "ds:SignatureValue",
                        )?)?
                    }
                    "ds:KeyInfo" => ds_key_info.set(crate::ds::KeyInfoType::read(
                        reader,
                        &attributes,
                        "ds:KeyInfo",
                    )?)?,
                    "ds:Object" => ds_object.push(crate::ds::ObjectType::read(
                        reader,
                        &attributes,
                        "ds:Object",
                    )?),
                    _ => return Err(xsd_api::ReadError::UnexpectedEvent),
                },
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
        Ok(SignatureType {
            ds_signed_info: ds_signed_info.require()?,
            ds_signature_value: ds_signature_value.require()?,
            ds_key_info: ds_key_info.get(),
            ds_object,
            id: id.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "SignatureType")?;
        SignatureType::read(reader, &attr, "ds:SignatureType")
    }
}

impl xsd_api::ReadXml for SignatureType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match SignatureType::read_top_level(&mut reader) {
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
