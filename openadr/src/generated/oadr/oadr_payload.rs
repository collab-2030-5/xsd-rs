use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OadrPayload {
    pub oadr_oadr_signed_object: crate::oadr::OadrSignedObject,
}

impl OadrPayload {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        self.oadr_oadr_signed_object.write_with_name(
            writer,
            "oadr:oadrSignedObject",
            false,
            false,
        )?;
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
            start.attr("xsi:type", "oadrPayload")
        } else {
            start
        };

        let start = start.ns("oadr", "http://openadr.org/oadr-2.0b/2012/07");
        let start = start.ns("ei", "http://docs.oasis-open.org/ns/energyinterop/201110");
        let start = start.ns(
            "pyld",
            "http://docs.oasis-open.org/ns/energyinterop/201110/payloads",
        );
        let start = start.ns("xcal", "urn:ietf:params:xml:ns:icalendar-2.0");
        let start = start.ns("emix", "http://docs.oasis-open.org/ns/emix/2011/06");
        let start = start.ns("strm", "urn:ietf:params:xml:ns:icalendar-2.0:stream");
        let start = start.ns("power", "http://docs.oasis-open.org/ns/emix/2011/06/power");
        let start = start.ns("gml", "http://www.opengis.net/gml/3.2");
        let start = start.ns(
            "scale",
            "http://docs.oasis-open.org/ns/emix/2011/06/siscale",
        );
        let start = start.ns("power", "http://docs.oasis-open.org/ns/emix/2011/06/power");

        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for OadrPayload {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "oadr:oadrPayload", true, false)?;
        Ok(())
    }
}

impl OadrPayload {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut oadr_oadr_signed_object: xsd_util::SetOnce<crate::oadr::OadrSignedObject> =
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
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "oadrSignedObject" => {
                        oadr_oadr_signed_object.set(crate::oadr::OadrSignedObject::read(
                            reader,
                            &attributes,
                            "oadrSignedObject",
                        )?)?
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
        Ok(OadrPayload {
            oadr_oadr_signed_object: oadr_oadr_signed_object.require()?,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "oadrPayload")?;
        OadrPayload::read(reader, &attr, "oadrPayload")
    }
}

impl xsd_api::ReadXml for OadrPayload {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match OadrPayload::read_top_level(&mut reader) {
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
