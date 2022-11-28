use crate::StringEnumeration;
use std::str::FromStr;
use xsd_api::ReadError;

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
                return if name.local_name.as_str() == parent_name {
                    Ok("".to_string())
                } else {
                    Err(ReadError::UnexpectedEvent)
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

pub fn read_string_enum<R, E>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> Result<E, ReadError>
where
    R: std::io::Read,
    E: StringEnumeration,
{
    let value = read_string(reader, parent_name)?;
    E::find(&value).ok_or(ReadError::UnknownEnumVariant)
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
                return if name.local_name.as_str() == parent_name {
                    Ok(())
                } else {
                    Err(ReadError::UnexpectedEvent)
                }
            }
            xml::reader::XmlEvent::Comment(_) => {}
            xml::reader::XmlEvent::Whitespace(_) => {}
            // anything else is an error
            _ => return Err(ReadError::UnexpectedEvent),
        }
    }
}

pub fn read_hex_bytes<R>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> Result<Vec<u8>, ReadError>
where
    R: std::io::Read,
{
    let value = read_string(reader, parent_name)?;
    parse_hex_bytes(&value)
}

pub fn read_type_from_string<R, T>(
    reader: &mut xml::reader::EventReader<R>,
    parent_name: &str,
) -> Result<T, ReadError>
where
    R: std::io::Read,
    T: FromStr,
    ReadError: From<T::Err>,
{
    let value = read_string(reader, parent_name)?;
    let value = T::from_str(&value)?;
    Ok(value)
}

fn parse_fixed_hex_bytes<const T: usize>(value: &str) -> Result<[u8; T], ReadError> {
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

fn parse_hex_bytes(value: &str) -> Result<Vec<u8>, ReadError> {
    let mut ret = Vec::new();
    iter_hex_bytes(value, |_, x| {
        ret.push(x);
        Ok(())
    })?;
    Ok(ret)
}

fn iter_hex_bytes<F>(value: &str, mut output: F) -> Result<usize, ReadError>
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
