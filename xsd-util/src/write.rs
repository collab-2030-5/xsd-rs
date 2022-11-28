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
    write_simple_element(writer, tag_name, hex.as_str())
}

pub fn write_simple_element<W>(
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
