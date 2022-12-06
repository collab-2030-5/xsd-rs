use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    pub ds_transforms: Option<crate::ds::TransformsType>,
    pub ds_digest_method: crate::ds::DigestMethodType,
    pub ds_digest_value: String,
    pub id: Option<String>,
    pub uri: Option<String>,
    pub typ: Option<String>,
}

impl ReferenceType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        if let Some(elem) = &self.ds_transforms {
            elem.write_with_name(writer, "ds:Transforms", false, false)?;
        }
        self.ds_digest_method
            .write_with_name(writer, "ds:DigestMethod", false, false)?;
        xsd_util::write_simple_element(writer, "ds:DigestValue", self.ds_digest_value.as_str())?;
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
        let start = match &self.uri {
            Some(attr) => start.attr("URI", attr.as_str()),
            None => start,
        };
        let start = match &self.typ {
            Some(attr) => start.attr("Type", attr.as_str()),
            None => start,
        };
        // ---- end attributes ----
        let start = if write_type {
            start.attr("xsi:type", "ds:ReferenceType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for ReferenceType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ds:ReferenceType", true, false)?;
        Ok(())
    }
}

impl ReferenceType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut ds_transforms: xsd_util::SetOnce<crate::ds::TransformsType> = Default::default();
        let mut ds_digest_method: xsd_util::SetOnce<crate::ds::DigestMethodType> =
            Default::default();
        let mut ds_digest_value: xsd_util::SetOnce<String> = Default::default();
        let mut id: xsd_util::SetOnce<String> = Default::default();
        let mut uri: xsd_util::SetOnce<String> = Default::default();
        let mut typ: xsd_util::SetOnce<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                "Id" => id.set(attr.value.clone())?,
                "URI" => uri.set(attr.value.clone())?,
                "Type" => typ.set(attr.value.clone())?,
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
                    "ds:Transforms" => ds_transforms.set(crate::ds::TransformsType::read(
                        reader,
                        &attributes,
                        "ds:Transforms",
                    )?)?,
                    "ds:DigestMethod" => ds_digest_method.set(
                        crate::ds::DigestMethodType::read(reader, &attributes, "ds:DigestMethod")?,
                    )?,
                    "ds:DigestValue" => {
                        ds_digest_value.set(xsd_util::read_string(reader, "ds:DigestValue")?)?
                    }
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
        Ok(ReferenceType {
            ds_transforms: ds_transforms.get(),
            ds_digest_method: ds_digest_method.require()?,
            ds_digest_value: ds_digest_value.require()?,
            id: id.get(),
            uri: uri.get(),
            typ: typ.get(),
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "ReferenceType")?;
        ReferenceType::read(reader, &attr, "ds:ReferenceType")
    }
}

impl xsd_api::ReadXml for ReferenceType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match ReferenceType::read_top_level(&mut reader) {
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
