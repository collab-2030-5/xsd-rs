use std::io::Write;

use xsd_model::config::*;
use xsd_model::resolved::*;

use crate::{FatalError, RustType};

use crate::gen::fields::ElementTransforms;
use crate::gen::traits::RustFieldName;
use crate::gen::*;
use heck::ToUpperCamelCase;
use xsd_model::{HexByteConstraints, PrimitiveType, WrapperType};

pub(crate) fn write(w: &mut dyn Write, st: &Struct) -> Result<(), FatalError> {
    writeln!(w, "use xml::writer::*;")?;
    writeln!(w, "use xml::common::Position;")?;
    writeln!(w)?;
    write_struct_definition(w, st)?;
    writeln!(w)?;
    write_serializers(w, st)?;
    writeln!(w)?;
    write_deserializer_impl(w, st)?;
    writeln!(w)?;
    write_deserializer_trait_impl(w, st)?;
    Ok(())
}

fn write_struct_definition(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    write_comment(w, &st.comment)?;
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub struct {} {{", st.id.name.to_upper_camel_case())?;
    indent(w, |w| write_struct_fields(w, st))?;
    writeln!(w, "}}")
}

fn write_serializers(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    // collect all the attribute fields
    let (attributes, elements) = split_fields(st);

    writeln!(w, "impl {} {{", st.id.name.to_upper_camel_case())?;
    indent(w, |w| {
        if !elements.is_empty() {
            writeln!(w, "fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
            indent(w, |w| {
                // write the elements
                for elem in &elements {
                    write_element(w, elem)?;
                }
                writeln!(w, "Ok(())")
            })?;
            writeln!(w, "}}")?;

            writeln!(w)?;
        }

        writeln!(w, "pub(crate) fn write_with_name<W>(&self, writer: &mut EventWriter<W>, name: &str, top_level: bool, write_type: bool) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(w, "let start = if top_level {{ super::add_schema_attr(events::XmlEvent::start_element(name)) }} else {{ events::XmlEvent::start_element(name) }};")?;

            if !attributes.is_empty() {
                writeln!(w, "// ---- start attributes ----")?;
                for att in &attributes {
                    write_attribute(w, att)?;
                }
                writeln!(w, "// ---- end attributes ----")?;
            }

            writeln!(w, "let start = if write_type {{")?;
            indent(w, |w| {
                writeln!(w, "start.attr(\"xsi:type\", \"{}\")", st.id.name)
            })?;
            writeln!(w, "}} else {{")?;
            indent(w, |w| writeln!(w, "start"))?;
            writeln!(w, "}};")?;

            writeln!(w, "writer.write(start)?;")?;

            if !elements.is_empty() {
                writeln!(w, "self.write_elem(writer)?;")?;
            }

            writeln!(w, "writer.write(events::XmlEvent::end_element())?;")?;
            writeln!(w, "Ok(())")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(
        w,
        "impl xsd_api::WriteXml for {} {{",
        st.id.name.to_upper_camel_case()
    )?;
    indent(w, |w| {
        writeln!(w, "fn write<W>(&self, config: xsd_api::WriteConfig, writer: &mut W) -> core::result::Result<(), xsd_api::WriteError> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let mut writer = config.build_xml_rs().create_writer(writer);"
            )?;
            writeln!(
                w,
                "self.write_with_name(&mut writer, \"{}\", true, false)?;",
                st.id
            )?;
            writeln!(w, "Ok(())")
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_deserializer_trait_impl(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    writeln!(
        w,
        "impl xsd_api::ReadXml for {} {{",
        st.id.name.to_upper_camel_case()
    )?;
    indent(w, |w| {
        writeln!(w, "fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "let mut reader = xml::reader::EventReader::new(r);")?;
            writeln!(w)?;
            writeln!(
                w,
                "match {}::read_top_level(&mut reader) {{",
                st.id.name.to_upper_camel_case()
            )?;
            indent(w, |w| {
                writeln!(w, "Ok(x) => Ok(x),")?;
                writeln!(w, "Err(err) => {{")?;
                indent(w, |w| {
                    writeln!(w, "let pos = reader.position();")?;
                    writeln!(
                        w,
                        "Err(xsd_api::ErrorWithLocation {{ err, line: pos.row, col: pos.column }})"
                    )
                })?;
                writeln!(w, "}}")
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")
}

fn write_deserializer_impl(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    let (attr, elem) = split_fields(st);

    writeln!(w, "impl {} {{", st.id.name.to_upper_camel_case())?;
    indent(w, |w| {
        writeln!(w, "pub(crate) fn read<R>(reader: &mut xml::reader::EventReader<R>, attrs: &Vec<xml::attribute::OwnedAttribute>, parent_tag: &str) -> core::result::Result<Self, xsd_api::ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "// one variable for each attribute and element")?;
            write_struct_cells(w, st)?;
            writeln!(w)?;
            write_attr_parse_loop(w, &attr)?;
            writeln!(w)?;
            write_elem_parse_loop(w, &elem)?;
            writeln!(w)?;
            writeln!(w, "// construct the type from the cells")?;
            writeln!(w, "Ok({} {{", st.id.name.to_upper_camel_case())?;
            indent(w, |w| write_struct_initializer(w, st))?;
            writeln!(w, "}})")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        writeln!(w, "fn read_top_level<R>(reader: &mut xml::reader::EventReader<R>) -> core::result::Result<Self, xsd_api::ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let attr = xsd_util::read_start_tag(reader, \"{}\")?;",
                &st.id.name
            )?;
            writeln!(
                w,
                "{}::read(reader, &attr, \"{}\")",
                st.id.name.to_upper_camel_case(),
                &st.id.name
            )
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_attr_parse_loop(w: &mut dyn Write, attrs: &[Attribute]) -> std::io::Result<()> {
    writeln!(w, "for attr in attrs.iter() {{")?;
    indent(w, |w| {
        writeln!(w, "match attr.name.local_name.as_str() {{")?;
        indent(w, |w| {
            for attr in attrs {
                writeln!(
                    w,
                    "\"{}\" => {}.set({})?,",
                    &attr.name,
                    attr.name.rust_field_name(),
                    parse_attribute(attr)
                )?;
            }
            writeln!(w, "_ => {{}}, // ignore unknown attributes")
        })?;
        writeln!(w, "}};")
    })?;
    writeln!(w, "}}")
}

fn write_struct_cells(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    for field in st.dedup_fields() {
        let cell_type = match &field.field_type {
            FieldType::Attribute(_, t) => format!("xsd_util::SetOnce<{}>", t.rust_struct_type()),
            FieldType::Element(m, t) => match m {
                ElemMultiplicity::Single | ElemMultiplicity::Optional => {
                    format!("xsd_util::SetOnce<{}>", t.rust_struct_type())
                }
                ElemMultiplicity::Vec => format!("Vec<{}>", t.rust_struct_type()),
            },
        };

        writeln!(
            w,
            "let mut {} : {} = Default::default();",
            field.name.rust_field_name(),
            cell_type
        )?;
    }

    Ok(())
}

fn write_elem_parse_loop(w: &mut dyn Write, elems: &[Element]) -> std::io::Result<()> {
    let start_elem_tag = {
        if elems.is_empty() {
            "xml::reader::XmlEvent::StartElement { .. }"
        } else {
            // are any of the elements structs?
            let has_struct = elems.iter().any(|e| e.field_type.is_struct());

            if has_struct {
                "xml::reader::XmlEvent::StartElement { name, attributes, .. }"
            } else {
                "xml::reader::XmlEvent::StartElement { name, .. }"
            }
        }
    };

    writeln!(w, "loop {{")?;
    indent(w, |w| {
        writeln!(w, "match reader.next()? {{")?;
        indent(w, |w| {
            writeln!(w, "xml::reader::XmlEvent::EndElement {{ name }} => {{")?;
            indent(w, |w| {
                writeln!(w, "if name.local_name.as_str() == parent_tag {{")?;
                indent(w, |w| {
                    writeln!(w, "// try to construct struct")?;
                    writeln!(w, "break;")
                })?;
                writeln!(w, "}} else {{")?;
                indent(w, |w| {
                    writeln!(w, "// TODO - make this more specific")?;
                    writeln!(w, "return Err(xsd_api::ReadError::UnexpectedEvent);")
                })?;
                writeln!(w, "}}")
            })?;
            writeln!(w, "}}")?;
            writeln!(w, "{} => {{", start_elem_tag)?;
            indent(w, |w| {
                if elems.is_empty() {
                    indent(w, |w| {
                        writeln!(w, "// this struct has no elements")?;
                        writeln!(w, "return Err(xsd_api::ReadError::UnexpectedEvent);")
                    })
                } else {
                    writeln!(w, "match name.local_name.as_str() {{")?;
                    indent(w, |w| {
                        for elem in elems {
                            writeln!(w, "\"{}\" => {{", &elem.bare_name())?;
                            indent(w, |w| write_element_handler(w, elem))?;
                            writeln!(w, "}}")?;
                        }
                        writeln!(w, "_ => return Err(xsd_api::ReadError::UnexpectedEvent)")
                    })?;
                    writeln!(w, "}}")
                }
            })?;
            writeln!(w, "}}")?;
            writeln!(w, "// treat these events as errors")?;
            writeln!(w, "xml::reader::XmlEvent::StartDocument {{ .. }} => return Err(xsd_api::ReadError::UnexpectedEvent),")?;
            writeln!(
                w,
                "xml::reader::XmlEvent::EndDocument => return Err(xsd_api::ReadError::UnexpectedEvent),"
            )?;
            writeln!(
                w,
                "xml::reader::XmlEvent::Characters(_) => return Err(xsd_api::ReadError::UnexpectedEvent),"
            )?;
            writeln!(w, "xml::reader::XmlEvent::ProcessingInstruction {{ .. }} => return Err(xsd_api::ReadError::UnexpectedEvent),")?;
            writeln!(w, "// ignore these events")?;
            writeln!(w, "xml::reader::XmlEvent::CData(_) => {{}}")?;
            writeln!(w, "xml::reader::XmlEvent::Comment(_) => {{}}")?;
            writeln!(w, "xml::reader::XmlEvent::Whitespace(_) => {{}}")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")
}

fn write_element_handler(w: &mut dyn Write, elem: &Element) -> std::io::Result<()> {
    let tx = elem.field_type.read_transform(&elem.bare_name());

    match &elem.multiplicity {
        ElemMultiplicity::Single | ElemMultiplicity::Optional => {
            writeln!(w, "{}.set({})?", elem.name.rust_field_name(), tx)
        }
        ElemMultiplicity::Vec => {
            writeln!(w, "{}.push({})", elem.name.rust_field_name(), tx)
        }
    }
}

fn write_struct_fields(writer: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    for field in st.dedup_fields() {
        let rust_type = field.field_type.rust_struct_type();

        write_comment(writer, &field.comment)?;

        writeln!(
            writer,
            "pub {}: {},",
            field.name.rust_field_name(),
            rust_type
        )?;
    }
    writeln!(writer)
}

fn write_struct_initializer(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    for field in st.dedup_fields() {
        let rust_var = field.name.rust_field_name();
        match &field.field_type {
            FieldType::Attribute(m, _) => match m {
                AttrMultiplicity::Single => {
                    writeln!(w, "{} : {}.require()?,", &rust_var, &rust_var)
                }
                AttrMultiplicity::Optional => writeln!(w, "{} : {}.get(),", &rust_var, &rust_var),
            },
            FieldType::Element(m, _) => match m {
                ElemMultiplicity::Single => {
                    writeln!(w, "{} : {}.require()?,", &rust_var, &rust_var)
                }
                ElemMultiplicity::Vec => writeln!(w, "{},", &rust_var),
                ElemMultiplicity::Optional => writeln!(w, "{} : {}.get(),", &rust_var, &rust_var),
            },
        }?;
    }

    Ok(())
}

fn split_fields(st: &Struct) -> (Vec<Attribute>, Vec<Element>) {
    let mut attrs = Vec::new();
    let mut elems = Vec::new();

    for field in st.dedup_fields() {
        match &field.field_type {
            FieldType::Attribute(m, t) => {
                let x = Attribute {
                    name: field.name.clone(),
                    field_type: t.clone(),
                    multiplicity: *m,
                };
                attrs.push(x);
            }
            FieldType::Element(m, t) => {
                let x = Element {
                    name: field.name.clone(),
                    field_type: t.clone(),
                    multiplicity: *m,
                };
                elems.push(x);
            }
        }
    }

    (attrs, elems)
}

enum AttributeTransform {
    Number,
    NumericEnum(Rc<NumericEnum<u8>>),
    NamedArray(Rc<NamedArray>),
    HexBitfield(Rc<BitField>),
    Duration(NumericDuration),
}

impl AttributeTransform {
    fn transform_to_string(&self, name: &str) -> String {
        match self {
            Self::Number => format!("{}.to_string()", name),
            Self::NumericEnum(_) => {
                format!("{}.value().to_string()", name)
            }
            Self::NamedArray(_) => {
                format!("to_hex({}.inner.as_slice())", name)
            }
            Self::HexBitfield(_) => {
                format!("{}.to_hex()", name)
            }
            Self::Duration(x) => match x {
                NumericDuration::Seconds(_) => {
                    format!("{}.as_secs().to_string()", name)
                }
            },
        }
    }
    fn parse_from_string(&self) -> String {
        match self {
            Self::Number => "attr.value.parse()?".to_string(),
            Self::NumericEnum(e) => {
                format!("structs::{}::from_value(attr.value.parse()?)", e.name)
            }
            Self::NamedArray(x) => {
                format!(
                    "structs::{} {{ inner: parse_fixed_hex_bytes(&attr.value)? }}",
                    x.name
                )
            }
            Self::HexBitfield(x) => {
                format!("structs::{}::from_hex(&attr.value)?", x.name)
            }
            Self::Duration(x) => match x {
                NumericDuration::Seconds(enc) => match enc {
                    DurationEncoding::UInt32 => {
                        "Duration::from_secs(u32::from_str_radix(&attr.value, 10)? as u64)"
                            .to_string()
                    }
                },
            },
        }
    }
}

fn get_attr_transform(attr_type: &SimpleType) -> Option<AttributeTransform> {
    match attr_type {
        SimpleType::Primitive(primitive) => match primitive {
            PrimitiveType::Boolean => Some(AttributeTransform::Number),
            PrimitiveType::HexBytes(HexByteConstraints::Single) => Some(AttributeTransform::Number),
            PrimitiveType::HexBytes(HexByteConstraints::Bytes { .. }) => None,
            PrimitiveType::String(_) => None,
            PrimitiveType::Number(_) => Some(AttributeTransform::Number),
            PrimitiveType::NumericDuration(x) => Some(AttributeTransform::Duration(*x)),
        },
        SimpleType::Wrapper(wrapper) => match wrapper {
            WrapperType::EnumU8(_, x) => Some(AttributeTransform::NumericEnum(x.clone())),
            WrapperType::NamedArray(_, x) => Some(AttributeTransform::NamedArray(x.clone())),
            WrapperType::HexBitField(_, x) => Some(AttributeTransform::HexBitfield(x.clone())),
            WrapperType::Enum(_) => unimplemented!(),
        },
    }
}

fn parse_attribute(attr: &Attribute) -> String {
    match get_attr_transform(&attr.field_type) {
        None => "attr.value.clone()".to_string(),
        Some(x) => x.parse_from_string(),
    }
}

fn write_attribute<W>(w: &mut W, attr: &Attribute) -> std::io::Result<()>
where
    W: Write,
{
    let name = attr.name.rust_field_name();
    let self_name = format!("self.{}", &name);
    let transform = get_attr_transform(&attr.field_type);

    match attr.multiplicity {
        AttrMultiplicity::Single => {
            if let Some(tx) = &transform {
                writeln!(w, "let {} = {};", &name, tx.transform_to_string(&self_name))?;
                writeln!(
                    w,
                    "let start = start.attr(\"{}\", {}.as_str());",
                    attr.name, &name
                )?;
            } else {
                writeln!(
                    w,
                    "let start = start.attr(\"{}\", {}.as_str());",
                    attr.name, &self_name
                )?;
            }
        }
        AttrMultiplicity::Optional => {
            let match_name = if let Some(tx) = &transform {
                writeln!(
                    w,
                    "let {} = self.{}.map(|x| {});",
                    &name,
                    &name,
                    tx.transform_to_string("x")
                )?;
                &name
            } else {
                &self_name
            };
            writeln!(w, "let start = match &{} {{", match_name)?;
            indent(w, |w| {
                writeln!(w, "Some(attr) => {{")?;
                indent(w, |w| {
                    writeln!(w, "start.attr(\"{}\", attr.as_str())", attr.name)
                })?;
                writeln!(w, "}},")?;
                writeln!(w, "None => start,")?;

                Ok(())
            })?;
            writeln!(w, "}};")?;
        }
    }

    Ok(())
}

fn write_element<W>(w: &mut W, elem: &Element) -> std::io::Result<()>
where
    W: Write,
{
    match &elem.multiplicity {
        ElemMultiplicity::Single => {
            let name = format!("self.{}", elem.name.rust_field_name());
            let tx = elem.field_type.write_transform(&name, &elem.name);
            writeln!(w, "{}", tx)?;
        }
        ElemMultiplicity::Vec => {
            let tx = elem.field_type.write_transform("item", &elem.name);
            writeln!(w, "for item in &self.{} {{", elem.name.rust_field_name())?;
            indent(w, |w| writeln!(w, "{}", tx))?;
            writeln!(w, "}}")?;
        }
        ElemMultiplicity::Optional => {
            writeln!(
                w,
                "if let Some(elem) = &self.{} {{",
                elem.name.to_snake_case()
            )?;
            let tx = elem.field_type.write_transform("elem", &elem.name);
            indent(w, |w| writeln!(w, "{}", tx))?;
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

struct Attribute {
    name: String,
    field_type: SimpleType,
    multiplicity: AttrMultiplicity,
}

struct Element {
    name: String,
    field_type: AnyType,
    multiplicity: ElemMultiplicity,
}

impl Element {
    pub fn bare_name(&self) -> String {
        match self.name.split_once(':') {
            None => self.name.to_owned(),
            Some((_ns, name)) => name.to_owned(),
        }
    }
}
