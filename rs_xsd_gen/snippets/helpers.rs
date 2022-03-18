struct SetOnce<T> {
    inner: Option<T>
}

impl<T> Default for SetOnce<T> {
    fn default() -> Self {
        Self {
            inner: None
        }
    }
}

impl<T> SetOnce<T> {

    fn set(&mut self, value: T) -> core::result::Result<(), ReadError> {
        if self.inner.is_some() {
            return Err(ReadError::DuplicateField);
        }
        self.inner = Some(value);
        Ok(())
    }

    fn require(self) -> core::result::Result<T, ReadError> {
        match self.inner {
            Some(x) => Ok(x),
            None => Err(ReadError::MissingMandatoryField),
        }
    }

    fn get(self) -> Option<T> {
        self.inner
    }
}

fn read_string<R>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> core::result::Result<String, ReadError>
    where
        R: std::io::Read,
{
    loop {
        match reader.next()? {
            xml::reader::XmlEvent::EndElement { name } => {
                if name.local_name.as_str() == parent_name {
                    return Ok("".to_string());
                } else {
                    return Err(ReadError::UnexpectedEvent);
                }
            }
            xml::reader::XmlEvent::Characters(value) => {
                // now find the end element
                expect_end_element(reader, parent_name)?;
                return Ok(value)
            },
            // we can ignore these
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // anything else is an error
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

fn expect_end_element<R>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> core::result::Result<(), ReadError>
    where
        R: std::io::Read,
{
    loop {
        match reader.next()? {
            xml::reader::XmlEvent::EndElement { name } => {
                if name.local_name.as_str() == parent_name {
                    return Ok(());
                } else {
                    return Err(ReadError::UnexpectedEvent);
                }
            }
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // anything else is an error
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

fn parse_hex_bytes(value: &str) -> core::result::Result<Vec<u8>, ReadError> {
    let mut ret = Vec::new();
    if value.len() % 2 != 0 {
        return Err(ReadError::BadHexString);
    }
    let count = value.len() / 2;
    for i in 0..count {
        let start = 2*i;
        let range = start..start+2;
        match value.get(range) {
            None => {
                return Err(ReadError::BadHexString);
            }
            Some(s) => {
                ret.push(u8::from_str_radix(s, 16)?)
            }
        }
    }

    Ok(ret)
}

fn read_start_tag<R>(reader: &mut xml::reader::EventReader<R>, type_name: &str) -> core::result::Result<Vec<xml::attribute::OwnedAttribute>, ReadError> where R: std::io::Read {
    expect_start_doc(reader)?;
    read_start_elem(reader, type_name)
}

fn expect_start_doc<R>(reader: &mut xml::reader::EventReader<R>) -> core::result::Result<(), ReadError> where R: std::io::Read {
    loop {
        match reader.next()? {
            xml::reader::XmlEvent::StartDocument { .. } => return Ok(()),
            // ignore
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // errors
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

fn read_start_elem<R>(reader: &mut xml::reader::EventReader<R>, type_name: &str) -> core::result::Result<Vec<xml::attribute::OwnedAttribute>, ReadError> where R: std::io::Read {
    loop {
        match reader.next()? {
            xml::reader::XmlEvent::StartElement { name, attributes, .. } => {
                if name.local_name.as_str() == type_name {
                    return Ok(attributes);
                } else {
                    // TODO - more descriptive
                    return Err(ReadError::UnexpectedEvent);
                }
            }
            // ignore
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // errors
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

fn write_simple_tag<W>(
    writer: &mut EventWriter<W>,
    tag_name: &str,
    data: &str,
) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
{
    writer.write(xml::writer::XmlEvent::start_element(tag_name))?;
    writer.write(data)?;
    writer.write(xml::writer::XmlEvent::end_element())
}

pub(crate) fn find_xsi_type(attrs: &[xml::attribute::OwnedAttribute]) -> core::result::Result<&str, crate::ReadError> {
    let result = attrs.iter().find_map(|x| {
        match (x.name.namespace.as_deref(), x.name.local_name.as_str()) {
            (Some("xsi"), "type") => Some(x.value.as_str()),
            _ => None,
        }
    });

    result.ok_or(crate::ReadError::UnknownType)
}
