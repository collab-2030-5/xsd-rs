use xml::EventWriter;

/*
pub trait WritePrimitive {
    fn write<W>(&self, writer: &mut xml::EventWriter<W>, tag_name: &str) -> Result<(), xml::writer::Error> where W: std::io::Write;
}

impl WritePrimitive for String {
    fn write<W>(&self, writer: &mut EventWriter<W>, tag_name: &str) -> Result<(), xml::writer::Error> where W: std::io::Write {
        write_simple_element(writer, tag_name, self.as_str())
    }
}
*/

pub(crate) fn to_hex(data: &[u8]) -> String {
    data.iter().map(|x| format!("{:02x}", x)).collect()
}

pub fn write_hex_tag<W>(
    writer: &mut EventWriter<W>,
    tag_name: &str,
    data: &[u8],
) -> Result<(), xml::writer::Error>
where
    W: std::io::Write,
{
    let hex: String = to_hex(data);
    write_simple_element(writer, tag_name, hex.as_str())
}

pub fn write_element_using_to_string<W, T>(
    writer: &mut EventWriter<W>,
    tag_name: &str,
    data: T,
) -> Result<(), xml::writer::Error>
where
    W: std::io::Write,
    T: Copy + ToString,
{
    let data = data.to_string();
    write_simple_element(writer, tag_name, data.as_str())
}

pub fn write_duration_as_seconds<W>(
    writer: &mut EventWriter<W>,
    tag_name: &str,
    duration: std::time::Duration,
) -> Result<(), xml::writer::Error>
where
    W: std::io::Write,
{
    write_element_using_to_string(writer, tag_name, duration.as_secs())
}

pub fn write_simple_element<W>(
    writer: &mut EventWriter<W>,
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
