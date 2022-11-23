use xsd_api::ReadError;

#[derive(Default)]
pub struct SetOnce<T> {
    inner: Option<T>,
}

impl<T> SetOnce<T> {
    pub fn set(&mut self, value: T) -> core::result::Result<(), ReadError> {
        if self.inner.is_some() {
            return Err(ReadError::DuplicateField);
        }
        self.inner = Some(value);
        Ok(())
    }

    pub fn require(self) -> core::result::Result<T, ReadError> {
        match self.inner {
            Some(x) => Ok(x),
            None => Err(ReadError::MissingMandatoryField),
        }
    }

    pub fn get(self) -> Option<T> {
        self.inner
    }
}

pub fn read_string<R>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> Result<String, ReadError>
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
                return Ok(value);
            }
            // we can ignore these
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // anything else is an error
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

pub fn expect_end_element<R>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> Result<(), ReadError>
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

pub fn iter_hex_bytes<F>(value: &str, mut output: F) -> Result<usize, ReadError>
where
    F: FnMut(usize, u8) -> Result<(), ReadError>,
{
    if value.len() % 2 != 0 {
        return Err(ReadError::BadHexString);
    }

    let mut pos = 0;
    let count = value.len() / 2;
    // consume bytes left to right

    for i in 0..count {
        let start = 2 * i;
        let range = start..start + 2;
        match value.get(range) {
            None => {
                return Err(ReadError::BadHexString);
            }
            Some(s) => {
                let value = u8::from_str_radix(s, 16)?;
                output(pos, value)?;
            }
        }
        pos += 1;
    }

    Ok(pos)
}

pub fn parse_hex_bytes(value: &str) -> Result<Vec<u8>, ReadError> {
    let mut ret = Vec::new();
    iter_hex_bytes(value, |_, x| {
        ret.push(x);
        Ok(())
    })?;
    Ok(ret)
}

pub fn parse_fixed_hex_bytes<const T: usize>(value: &str) -> Result<[u8; T], ReadError> {
    let mut ret: [u8; T] = [0; T];

    iter_hex_bytes(value, |pos, byte| match ret.get_mut(pos) {
        None => Err(ReadError::BadHexString),
        Some(x) => {
            *x = byte;
            Ok(())
        }
    })?;

    Ok(ret)
}

pub fn read_start_tag<R>(
    reader: &mut xml::reader::EventReader<R>,
    type_name: &str,
) -> Result<Vec<xml::attribute::OwnedAttribute>, ReadError>
where
    R: std::io::Read,
{
    expect_start_doc(reader)?;
    read_start_elem(reader, type_name)
}

pub fn expect_start_doc<R>(reader: &mut xml::reader::EventReader<R>) -> Result<(), ReadError>
where
    R: std::io::Read,
{
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

pub fn read_start_elem<R>(
    reader: &mut xml::reader::EventReader<R>,
    type_name: &str,
) -> Result<Vec<xml::attribute::OwnedAttribute>, ReadError>
where
    R: std::io::Read,
{
    loop {
        match reader.next()? {
            xml::reader::XmlEvent::StartElement {
                name, attributes, ..
            } => {
                return if name.local_name.as_str() == type_name {
                    Ok(attributes)
                } else {
                    // TODO - more descriptive
                    Err(ReadError::UnexpectedEvent)
                };
            }
            // ignore
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // errors
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

pub(crate) fn to_hex(data: &[u8]) -> String {
    data.iter().map(|x| format!("{:02x}", x)).collect()
}

pub fn write_hex_tag<W>(
    writer: &mut xml::EventWriter<W>,
    tag_name: &str,
    data: &[u8],
) -> Result<(), xml::writer::Error>
where
    W: std::io::Write,
{
    let hex: String = to_hex(data);
    write_simple_tag(writer, tag_name, hex.as_str())
}

pub fn write_simple_tag<W>(
    writer: &mut xml::EventWriter<W>,
    tag_name: &str,
    data: &str,
) -> Result<(), xml::writer::Error>
where
    W: std::io::Write,
{
    writer.write(xml::writer::XmlEvent::start_element(tag_name))?;
    writer.write(data)?;
    writer.write(xml::writer::XmlEvent::end_element())
}

pub fn find_xsi_type(attrs: &[xml::attribute::OwnedAttribute]) -> Result<&str, ReadError> {
    let result =
        attrs.iter().find_map(
            |x| match (x.name.prefix.as_deref(), x.name.local_name.as_str()) {
                (Some("xsi"), "type") => Some(x.value.as_str()),
                (x, y) => {
                    println!("{:?}, {:?}", x, y);
                    None
                }
            },
        );

    result.ok_or(ReadError::MissingXsiType)
}
