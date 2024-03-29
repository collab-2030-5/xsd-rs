pub(crate) mod config;
pub(crate) mod parse;
pub(crate) mod traits;

use structopt::StructOpt;

use heck::{ToSnakeCase, ToUpperCamelCase};
use indent_write::io::IndentWriter;
use std::io::LineWriter;
use std::io::Write;
use std::path::{Path, PathBuf};
use xml_model::resolved::{
    AttrMultiplicity, ElemMultiplicity, ElementType, FieldType, Model, Struct,
};

use crate::config::BaseTypeConfig;
use crate::traits::RustType;
use std::rc::Rc;
use xml_model::config::{
    BitField, DurationEncoding, NamedArray, NumericDuration, NumericEnum, SubstitutedType,
};
use xml_model::SimpleType;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rs-xsd-gen",
    about = "Reads model of xml from json and emits rust bindings"
)]
struct Opt {
    /// input XSD file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
    /// config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
    /// rust output directory
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
    /// rust output directory
    #[structopt(short = "r", long = "remove")]
    remove_dir: bool,
}

type FatalError = Box<dyn std::error::Error>;

fn main() -> Result<(), FatalError> {
    let opt: Opt = Opt::from_args();
    let xsd = std::fs::read_to_string(opt.input)?;
    let config: config::Config = serde_json::from_reader(std::fs::File::open(opt.config)?)?;
    let model: xml_model::unresolved::UnresolvedModel = crate::parse::transform(&xsd);
    let model = model.resolve(config.mappings);

    create_main_output_dir(&opt.output, opt.remove_dir)?;

    write_model(opt.output, &model, &config.base_types)?;

    Ok(())
}

fn write_model(dir: PathBuf, model: &Model, config: &BaseTypeConfig) -> Result<(), FatalError> {
    let files = [
        ("config.rs", include_str!("../snippets/config.rs")),
        ("error.rs", include_str!("../snippets/error.rs")),
        ("helpers.rs", include_str!("../snippets/helpers.rs")),
        ("mod.rs", include_str!("../snippets/mod.rs")),
        ("traits.rs", include_str!("../snippets/traits.rs")),
    ];

    for (file_name, data) in files {
        write_file(&dir.join(file_name), data)?;
    }

    let struct_dir = dir.join("structs");
    std::fs::create_dir(&struct_dir)?;

    let mod_file = struct_dir.join("mod.rs");
    write_struct_mod_file(&mod_file, model)?;

    for (substituted, name) in model
        .substituted_types
        .iter()
        .filter_map(|x| x.type_name().map(|name| (x, name)))
    {
        let path = struct_dir.join(format!("{}.rs", name.to_snake_case()));
        let mut writer = create(&path)?;
        match substituted {
            SubstitutedType::NamedArray(x) => {
                write_named_array_file(&mut writer, x)?;
            }
            SubstitutedType::NumericEnum(x) => {
                write_enum_file(&mut writer, x)?;
            }
            SubstitutedType::HexBitField(x) => {
                write_bit_field_file(&mut writer, x)?;
            }
            SubstitutedType::NumericDuration(_) => {}
        }
    }

    for st in &model.structs {
        let path = struct_dir.join(format!("{}.rs", st.name.to_snake_case()));
        let mut writer = create(&path)?;
        write_struct_file(&mut writer, st)?;
    }

    let base_enum_dir = dir.join("base");
    write_base_enums(&base_enum_dir, model, config)?;

    Ok(())
}

fn write_struct_definition(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    write_comment(w, &st.comment)?;
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub struct {} {{", st.name.to_upper_camel_case())?;
    indent(w, |w| write_struct_fields(w, st))?;
    writeln!(w, "}}")
}

fn write_named_array_file(w: &mut dyn Write, na: &NamedArray) -> Result<(), FatalError> {
    writeln!(w, "#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]")?;
    writeln!(w, "pub struct {} {{", na.name)?;
    indent(w, |w| writeln!(w, "pub(crate) inner: [u8; Self::SIZE],"))?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl {} {{", na.name)?;
    indent(w, |w| writeln!(w, "pub const SIZE: usize = {};", na.size))?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_enum_file(w: &mut dyn Write, e: &NumericEnum<u8>) -> Result<(), FatalError> {
    writeln!(w, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]")?;
    writeln!(w, "pub enum {} {{", e.name)?;
    indent(w, |w| {
        for value in e.variants.values() {
            write_comment(w, &value.comment)?;
            writeln!(w, "{},", value.name)?;
        }
        writeln!(w, "/// Value not defined in the XSD")?;
        writeln!(w, "Other(u8)")
    })?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl {} {{", e.name)?;
    indent(w, |w| {
        writeln!(w, "pub fn value(&self) -> u8 {{")?;
        indent(w, |w| {
            writeln!(w, "match self {{")?;
            indent(w, |w| {
                for (value, var) in e.variants.iter() {
                    writeln!(w, "Self::{} => {},", var.name, value)?;
                }
                writeln!(w, "Self::Other(x) => *x,")?;
                Ok(())
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;

        writeln!(w)?;

        writeln!(w, "pub fn from_value(value: u8) -> Self {{")?;
        indent(w, |w| {
            writeln!(w, "match value {{")?;
            indent(w, |w| {
                for (value, var) in e.variants.iter() {
                    writeln!(w, "{} => Self::{},", value, var.name)?;
                }
                writeln!(w, "_ => Self::Other(value),")?;
                Ok(())
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_bit_field_file(w: &mut dyn Write, bf: &BitField) -> Result<(), FatalError> {
    write_comment(w, &bf.comment)?;
    writeln!(
        w,
        "#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]"
    )?;
    writeln!(w, "pub struct {} {{", bf.name)?;
    indent(w, |w| {
        for (num, byte) in bf.bytes.iter().enumerate() {
            writeln!(w, "// --- Byte #{} ---", num)?;
            writeln!(w)?;
            for (x, bit) in byte.iter() {
                writeln!(w, "// --- mask 0b{0:08b} ---", x)?;
                writeln!(w, "/// {}", bit.comment)?;
                writeln!(w, "pub {}: bool,", bit.name)?;
            }
        }
        Ok(())
    })?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl {} {{", bf.name)?;
    indent(w, |w| {
        writeln!(
            w,
            "pub(crate) fn from_hex(hex: &str) -> Result<Self, crate::ReadError> {{"
        )?;
        indent(w, |w| {
            writeln!(
                w,
                "let bytes = crate::parse_fixed_hex_bytes::<{}>(hex)?;",
                bf.bytes.len()
            )?;
            writeln!(w)?;
            writeln!(w, "let mut value: Self = Default::default();")?;
            writeln!(w)?;
            for (num, byte) in bf.bytes.iter().enumerate() {
                for (mask, bit) in byte.iter() {
                    writeln!(w, "if bytes[{}] & 0b{:08b} != 0 {{", num, mask)?;
                    indent(w, |w| writeln!(w, "value.{} = true;", bit.name))?;
                    writeln!(w, "}}")?;
                }
            }
            writeln!(w)?;
            writeln!(w, "Ok(value)")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        writeln!(w, "pub(crate) fn to_hex(&self) -> String {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let mut bytes : [u8; {}] = [0; {}];",
                bf.bytes.len(),
                bf.bytes.len()
            )?;
            writeln!(w)?;
            for (num, byte) in bf.bytes.iter().enumerate() {
                for (mask, bit) in byte.iter() {
                    writeln!(w, "if self.{} {{", bit.name)?;
                    indent(w, |w| writeln!(w, "bytes[{}] |= 0b{:08b};", num, mask))?;
                    writeln!(w, "}}")?;
                }
            }
            writeln!(w)?;
            writeln!(w, "crate::to_hex(&bytes)")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_struct_file(w: &mut dyn Write, st: &Struct) -> Result<(), FatalError> {
    writeln!(w, "use super::super::*;")?;
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

fn write_struct_mod_file(path: &Path, model: &Model) -> Result<(), FatalError> {
    let mut w = create(path)?;

    for st in model.structs.iter() {
        writeln!(w, "mod {};", st.name.to_snake_case())?;
    }

    for name in model.substituted_types.iter().filter_map(|x| x.type_name()) {
        writeln!(w, "mod {};", name.to_snake_case())?;
    }

    writeln!(w)?;

    for st in model.structs.iter() {
        writeln!(w, "pub use {}::*;", st.name.to_snake_case())?;
    }

    for name in model.substituted_types.iter().filter_map(|x| x.type_name()) {
        writeln!(w, "pub use {}::*;", name.to_snake_case())?;
    }

    writeln!(w)?;

    write_add_schema_attr(&mut w, model)?;

    Ok(())
}

fn create_main_output_dir(path: &Path, delete_dir: bool) -> Result<(), FatalError> {
    if path.exists() {
        if path.is_file() {
            return Err(format!(
                "Output must be a directory, but the supplied path is a file: {:?}",
                path
            )
            .into());
        }

        if path.is_dir() {
            if delete_dir {
                std::fs::remove_dir_all(path)?;
            } else {
                return Err(format!("Cannot write into existing directory {:?}. Delete the directory or use the -r flag to remove it", path).into());
            }
        }
    }

    std::fs::create_dir(path)?;

    Ok(())
}

fn create(path: &std::path::Path) -> Result<impl std::io::Write, FatalError> {
    let output = std::fs::File::create(path)?;
    Ok(LineWriter::new(output))
}

fn write_comment(w: &mut dyn Write, comment: &Option<String>) -> std::io::Result<()> {
    if let Some(comment) = comment {
        for line in comment.lines() {
            writeln!(w, "/// {}", line.replace('\t', "    "))?;
        }
    }
    Ok(())
}

fn indent<F>(w: &mut dyn Write, f: F) -> std::io::Result<()>
where
    F: Fn(&mut IndentWriter<&mut dyn Write>) -> std::io::Result<()>,
{
    let mut w = IndentWriter::new("    ", w);
    f(&mut w)
}

fn get_rust_field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    match snake.as_str() {
        // have to rename reserved identifiers
        "type" => "typ".to_string(),
        _ => snake,
    }
}

fn write_struct_fields(writer: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    if let Some(bt) = &st.base_type {
        write_struct_fields(writer, bt)?;
    }

    writeln!(writer)?;
    writeln!(writer, "// --- these fields come from {} ---", st.name)?;
    writeln!(writer)?;
    for field in &st.fields {
        let rust_type = field.field_type.rust_struct_type();

        write_comment(writer, &field.comment)?;

        writeln!(
            writer,
            "pub {}: {},",
            get_rust_field_name(&field.name),
            rust_type
        )?;
    }
    writeln!(writer)
}

struct Attribute {
    name: String,
    field_type: SimpleType,
    multiplicity: AttrMultiplicity,
}

struct Element {
    name: String,
    field_type: ElementType,
    multiplicity: ElemMultiplicity,
}

fn split_fields(st: &Struct) -> (Vec<Attribute>, Vec<Element>) {
    let mut attrs = Vec::new();
    let mut elems = Vec::new();

    if let Some(base) = &st.base_type {
        let (attr, elem) = split_fields(base);
        for field in attr {
            attrs.push(field);
        }
        for field in elem {
            elems.push(field);
        }
    }

    for field in &st.fields {
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
    Enum(std::rc::Rc<NumericEnum<u8>>),
    NamedArray(std::rc::Rc<NamedArray>),
    HexBitfield(std::rc::Rc<BitField>),
    Duration(std::rc::Rc<NumericDuration>),
}

impl AttributeTransform {
    fn transform_to_string(&self, name: &str) -> String {
        match self {
            Self::Number => format!("{}.to_string()", name),
            Self::Enum(_) => {
                format!("{}.value().to_string()", name)
            }
            Self::NamedArray(_) => {
                format!("to_hex({}.inner.as_slice())", name)
            }
            Self::HexBitfield(_) => {
                format!("{}.to_hex()", name)
            }
            Self::Duration(x) => match x.as_ref() {
                NumericDuration::Seconds(_) => {
                    format!("{}.as_secs().to_string()", name)
                }
            },
        }
    }
    fn parse_from_string(&self) -> String {
        match self {
            Self::Number => "attr.value.parse()?".to_string(),
            Self::Enum(e) => {
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
            AttributeTransform::Duration(x) => match x.as_ref() {
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
        SimpleType::Boolean => Some(AttributeTransform::Number),
        SimpleType::HexByte => Some(AttributeTransform::Number),
        SimpleType::HexBytes(_) => None,
        SimpleType::String(_) => None,
        SimpleType::I8(_) => Some(AttributeTransform::Number),
        SimpleType::U8(_) => Some(AttributeTransform::Number),
        SimpleType::I16(_) => Some(AttributeTransform::Number),
        SimpleType::U16(_) => Some(AttributeTransform::Number),
        SimpleType::I32(_) => Some(AttributeTransform::Number),
        SimpleType::U32(_) => Some(AttributeTransform::Number),
        SimpleType::I64(_) => Some(AttributeTransform::Number),
        SimpleType::U64(_) => Some(AttributeTransform::Number),
        SimpleType::EnumU8(x) => Some(AttributeTransform::Enum(x.clone())),
        SimpleType::NamedArray(x) => Some(AttributeTransform::NamedArray(x.clone())),
        SimpleType::HexBitField(x) => Some(AttributeTransform::HexBitfield(x.clone())),
        SimpleType::NumericDuration(x) => Some(AttributeTransform::Duration(x.clone())),
    }
}

enum ElementTransform {
    Struct(std::rc::Rc<Struct>),
    Number,
    String,
    HexBytes,
    Enum(std::rc::Rc<xml_model::config::NumericEnum<u8>>),
    NamedHexArray(std::rc::Rc<NamedArray>),
    HexBitField(std::rc::Rc<BitField>),
    NumericDuration(std::rc::Rc<NumericDuration>),
}

impl ElementTransform {
    fn write_value<W>(&self, w: &mut W, rust_name: &str, xsd_name: &str) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            ElementTransform::Struct(x) => {
                if x.metadata.is_base {
                    writeln!(
                        w,
                        "{}.write_with_name(writer, \"{}\")?;",
                        rust_name, xsd_name
                    )
                } else {
                    writeln!(
                        w,
                        "{}.write_with_name(writer, \"{}\", false, false)?;",
                        rust_name, xsd_name
                    )
                }
            }
            ElementTransform::String => {
                writeln!(
                    w,
                    "write_simple_tag(writer, \"{}\", {}.as_str())?;",
                    xsd_name, rust_name
                )
            }
            ElementTransform::Number => {
                writeln!(w, "let value = {}.to_string();", rust_name)?;
                writeln!(
                    w,
                    "write_simple_tag(writer, \"{}\", value.as_str())?;",
                    xsd_name
                )
            }
            ElementTransform::HexBytes => {
                writeln!(
                    w,
                    "write_hex_tag(writer, \"{}\", &{})?;",
                    xsd_name, rust_name
                )
            }
            ElementTransform::Enum(_) => {
                writeln!(w, "let value = {}.value().to_string();", rust_name)?;
                writeln!(
                    w,
                    "write_simple_tag(writer, \"{}\", value.as_str())?;",
                    xsd_name
                )
            }
            ElementTransform::NamedHexArray(_) => {
                writeln!(
                    w,
                    "write_hex_tag(writer, \"{}\", {}.inner.as_slice())?;",
                    xsd_name, rust_name
                )
            }
            ElementTransform::HexBitField(_) => {
                writeln!(
                    w,
                    "write_simple_tag(writer, \"{}\", &{}.to_hex())?;",
                    xsd_name, rust_name
                )
            }
            ElementTransform::NumericDuration(x) => match x.as_ref() {
                NumericDuration::Seconds(_) => {
                    writeln!(
                        w,
                        "write_simple_tag(writer, \"{}\", &{}.as_secs().to_string())?;",
                        xsd_name, rust_name
                    )
                }
            },
        }
    }
}

fn get_simple_type_transform(st: &SimpleType) -> ElementTransform {
    match st {
        SimpleType::Boolean => ElementTransform::Number,
        SimpleType::HexByte => ElementTransform::Number,
        SimpleType::HexBytes(_) => ElementTransform::HexBytes,
        SimpleType::String(_) => ElementTransform::String,
        SimpleType::I8(_) => ElementTransform::Number,
        SimpleType::U8(_) => ElementTransform::Number,
        SimpleType::I16(_) => ElementTransform::Number,
        SimpleType::U16(_) => ElementTransform::Number,
        SimpleType::I32(_) => ElementTransform::Number,
        SimpleType::U32(_) => ElementTransform::Number,
        SimpleType::I64(_) => ElementTransform::Number,
        SimpleType::U64(_) => ElementTransform::Number,
        SimpleType::EnumU8(x) => ElementTransform::Enum(x.clone()),
        SimpleType::NamedArray(x) => ElementTransform::NamedHexArray(x.clone()),
        SimpleType::HexBitField(x) => ElementTransform::HexBitField(x.clone()),
        SimpleType::NumericDuration(x) => ElementTransform::NumericDuration(x.clone()),
    }
}

fn get_elem_transform(elem_type: &ElementType) -> ElementTransform {
    match elem_type {
        ElementType::Simple(s) => get_simple_type_transform(s),
        ElementType::Struct(s) => ElementTransform::Struct(s.clone()),
    }
}

fn write_element<W>(w: &mut W, elem: &Element) -> std::io::Result<()>
where
    W: Write,
{
    let transform = get_elem_transform(&elem.field_type);

    match &elem.multiplicity {
        ElemMultiplicity::Single => {
            let name = format!("self.{}", get_rust_field_name(&elem.name));
            transform.write_value(w, &name, &elem.name)?;
        }
        ElemMultiplicity::Vec => {
            writeln!(
                w,
                "for item in &self.{} {{",
                get_rust_field_name(&elem.name)
            )?;
            indent(w, |w| transform.write_value(w, "item", &elem.name))?;
            writeln!(w, "}}")?;
        }
        ElemMultiplicity::Optional => {
            writeln!(
                w,
                "if let Some(elem) = &self.{} {{",
                elem.name.to_snake_case()
            )?;
            indent(w, |w| transform.write_value(w, "elem", &elem.name))?;
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

fn write_attribute<W>(w: &mut W, attr: &Attribute) -> std::io::Result<()>
where
    W: Write,
{
    let name = get_rust_field_name(&attr.name);
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

fn write_add_schema_attr(w: &mut dyn Write, model: &Model) -> std::io::Result<()> {
    let target_ns = model.target_ns.as_ref().expect("requires target namespace");

    writeln!(
        w,
        "pub(crate) fn add_schema_attr(start: xml::writer::events::StartElementBuilder) -> xml::writer::events::StartElementBuilder {{"
    )?;
    indent(w, |w| {
        writeln!(w, "start")?;
        indent(w, |w| {
            writeln!(
                w,
                ".attr(\"xmlns:xsi\", \"http://www.w3.org/2001/XMLSchema-instance\")"
            )?;
            writeln!(
                w,
                ".attr(\"xmlns:xsd\", \"http://www.w3.org/2001/XMLSchema\")"
            )?;
            writeln!(w, ".attr(\"xmlns\", \"{}\")", target_ns.uri)
        })?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_serializers(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    // collect all the attribute fields
    let (attributes, elements) = split_fields(st);

    writeln!(w, "impl {} {{", st.name.to_upper_camel_case())?;
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
            writeln!(w, "let start = if top_level {{ structs::add_schema_attr(events::XmlEvent::start_element(name)) }} else {{ events::XmlEvent::start_element(name) }};")?;

            if !attributes.is_empty() {
                writeln!(w, "// ---- start attributes ----")?;
                for att in &attributes {
                    write_attribute(w, att)?;
                }
                writeln!(w, "// ---- end attributes ----")?;
            }

            writeln!(w, "let start = if write_type {{")?;
            indent(w, |w| {
                writeln!(w, "start.attr(\"xsi:type\", \"{}\")", st.name)
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

    writeln!(w, "impl WriteXml for {} {{", st.name.to_upper_camel_case())?;
    indent(w, |w| {
        writeln!(w, "fn write<W>(&self, config: WriteConfig, writer: &mut W) -> core::result::Result<(), WriteError> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let mut writer = config.to_xml_rs().create_writer(writer);"
            )?;
            writeln!(
                w,
                "self.write_with_name(&mut writer, \"{}\", true, false)?;",
                st.name
            )?;
            writeln!(w, "Ok(())")
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_file(path: &Path, data: &str) -> Result<(), FatalError> {
    let mut writer = create(path)?;
    writer.write_all(data.as_bytes())?;
    Ok(())
}

fn write_struct_cells(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    // do this recursively depth-first
    if let Some(bt) = &st.base_type {
        write_struct_cells(w, bt)?;
    }

    for field in &st.fields {
        let cell_type = match &field.field_type {
            FieldType::Attribute(_, t) => format!("SetOnce<{}>", t.rust_struct_type()),
            FieldType::Element(m, t) => match m {
                ElemMultiplicity::Single | ElemMultiplicity::Optional => {
                    format!("SetOnce<{}>", t.rust_struct_type())
                }
                ElemMultiplicity::Vec => format!("Vec<{}>", t.rust_struct_type()),
            },
        };

        writeln!(
            w,
            "let mut {} : {} = Default::default();",
            get_rust_field_name(&field.name),
            cell_type
        )?;
    }

    Ok(())
}

fn write_struct_initializer(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    // do this recursively depth-first
    if let Some(bt) = &st.base_type {
        write_struct_initializer(w, bt)?;
    }

    for field in &st.fields {
        let rust_var = get_rust_field_name(&field.name);
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

fn parse_attribute(attr: &Attribute) -> String {
    match get_attr_transform(&attr.field_type) {
        None => "attr.value.clone()".to_string(),
        Some(x) => x.parse_from_string(),
    }
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
                    get_rust_field_name(&attr.name),
                    parse_attribute(attr)
                )?;
            }
            writeln!(w, "_ => {{}}, // ignore unknown attributes")
        })?;
        writeln!(w, "}};")
    })?;
    writeln!(w, "}}")
}

fn write_element_handler(w: &mut dyn Write, elem: &Element) -> std::io::Result<()> {
    let transform = get_elem_transform(&elem.field_type);

    let tx: String = match transform {
        ElementTransform::Struct(s) => {
            if s.metadata.is_base {
                format!(
                    "base::{}::read(reader, &attributes, \"{}\")?",
                    s.name.to_upper_camel_case(),
                    &elem.name
                )
            } else {
                format!(
                    "structs::{}::read(reader, &attributes, \"{}\")?",
                    s.name.to_upper_camel_case(),
                    &elem.name
                )
            }
        }
        ElementTransform::Number => {
            format!("read_string(reader, \"{}\")?.parse()?", &elem.name)
        }
        ElementTransform::String => {
            format!("read_string(reader, \"{}\")?", &elem.name)
        }
        ElementTransform::HexBytes => format!(
            "parse_hex_bytes(&read_string(reader, \"{}\")?)?",
            &elem.name
        ),
        ElementTransform::Enum(x) => {
            format!(
                "structs::{}::from_value(read_string(reader, \"{}\")?.parse()?)",
                x.name, &elem.name
            )
        }
        ElementTransform::NamedHexArray(buff) => format!(
            "structs::{} {{ inner: parse_fixed_hex_bytes::<{}>(&read_string(reader, \"{}\")?)? }}",
            buff.name, buff.size, &elem.name
        ),
        ElementTransform::HexBitField(x) => format!(
            "structs::{}::from_hex(&read_string(reader, \"{}\")?)?",
            x.name, elem.name,
        ),
        ElementTransform::NumericDuration(x) => match x.as_ref() {
            NumericDuration::Seconds(_) => {
                format!(
                    "std::time::Duration::from_secs(read_string(reader, \"{}\")?.parse()?)",
                    &elem.name
                )
            }
        },
    };

    match &elem.multiplicity {
        ElemMultiplicity::Single | ElemMultiplicity::Optional => {
            writeln!(w, "{}.set({})?", get_rust_field_name(&elem.name), tx)
        }
        ElemMultiplicity::Vec => {
            writeln!(w, "{}.push({})", get_rust_field_name(&elem.name), tx)
        }
    }
}

fn write_elem_parse_loop(w: &mut dyn Write, elems: &[Element]) -> std::io::Result<()> {
    let start_elem_tag = {
        if elems.is_empty() {
            "xml::reader::XmlEvent::StartElement { .. }"
        } else {
            // are any of the elements structs?
            let has_struct = elems
                .iter()
                .any(|e| match get_elem_transform(&e.field_type) {
                    ElementTransform::Struct(_) => true,
                    ElementTransform::Number => false,
                    ElementTransform::String => false,
                    ElementTransform::HexBytes => false,
                    ElementTransform::Enum(_) => false,
                    ElementTransform::NamedHexArray(_) => false,
                    ElementTransform::HexBitField(_) => false,
                    ElementTransform::NumericDuration(_) => false,
                });

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
                    writeln!(w, "return Err(ReadError::UnexpectedEvent);")
                })?;
                writeln!(w, "}}")
            })?;
            writeln!(w, "}}")?;
            writeln!(w, "{} => {{", start_elem_tag)?;
            indent(w, |w| {
                if elems.is_empty() {
                    indent(w, |w| {
                        writeln!(w, "// this struct has no elements")?;
                        writeln!(w, "return Err(ReadError::UnexpectedEvent);")
                    })
                } else {
                    writeln!(w, "match name.local_name.as_str() {{")?;
                    indent(w, |w| {
                        for elem in elems {
                            writeln!(w, "\"{}\" => {{", &elem.name)?;
                            indent(w, |w| write_element_handler(w, elem))?;
                            writeln!(w, "}}")?;
                        }
                        writeln!(w, "_ => return Err(ReadError::UnexpectedEvent)")
                    })?;
                    writeln!(w, "}}")
                }
            })?;
            writeln!(w, "}}")?;
            writeln!(w, "// treat these events as errors")?;
            writeln!(w, "xml::reader::XmlEvent::StartDocument {{ .. }} => return Err(ReadError::UnexpectedEvent),")?;
            writeln!(
                w,
                "xml::reader::XmlEvent::EndDocument => return Err(ReadError::UnexpectedEvent),"
            )?;
            writeln!(
                w,
                "xml::reader::XmlEvent::Characters(_) => return Err(ReadError::UnexpectedEvent),"
            )?;
            writeln!(w, "xml::reader::XmlEvent::ProcessingInstruction {{ .. }} => return Err(ReadError::UnexpectedEvent),")?;
            writeln!(w, "// ignore these events")?;
            writeln!(w, "xml::reader::XmlEvent::CData(_) => {{}}")?;
            writeln!(w, "xml::reader::XmlEvent::Comment(_) => {{}}")?;
            writeln!(w, "xml::reader::XmlEvent::Whitespace(_) => {{}}")
        })?;
        writeln!(w, "}}")
    })?;
    writeln!(w, "}}")
}

fn write_deserializer_trait_impl(w: &mut dyn Write, st: &Struct) -> std::io::Result<()> {
    writeln!(w, "impl ReadXml for {} {{", st.name.to_upper_camel_case())?;
    indent(w, |w| {
        writeln!(w, "fn read<R>(r: &mut R) -> core::result::Result<Self, ErrorWithLocation> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "let mut reader = xml::reader::EventReader::new(r);")?;
            writeln!(w)?;
            writeln!(
                w,
                "match {}::read_top_level(&mut reader) {{",
                st.name.to_upper_camel_case()
            )?;
            indent(w, |w| {
                writeln!(w, "Ok(x) => Ok(x),")?;
                writeln!(w, "Err(err) => {{")?;
                indent(w, |w| {
                    writeln!(w, "let pos = reader.position();")?;
                    writeln!(
                        w,
                        "Err(ErrorWithLocation {{ err, line: pos.row, col: pos.column }})"
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

    writeln!(w, "impl {} {{", st.name.to_upper_camel_case())?;
    indent(w, |w| {
        writeln!(w, "pub(crate) fn read<R>(reader: &mut xml::reader::EventReader<R>, attrs: &Vec<xml::attribute::OwnedAttribute>, parent_tag: &str) -> core::result::Result<Self, ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "// one variable for each attribute and element")?;
            write_struct_cells(w, st)?;
            writeln!(w)?;
            write_attr_parse_loop(w, &attr)?;
            writeln!(w)?;
            write_elem_parse_loop(w, &elem)?;
            writeln!(w)?;
            writeln!(w, "// construct the type from the cells")?;
            writeln!(w, "Ok({} {{", st.name.to_upper_camel_case())?;
            indent(w, |w| write_struct_initializer(w, st))?;
            writeln!(w, "}})")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        writeln!(w, "fn read_top_level<R>(reader: &mut xml::reader::EventReader<R>) -> core::result::Result<Self, ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "let attr = read_start_tag(reader, \"{}\")?;", &st.name)?;
            writeln!(
                w,
                "{}::read(reader, &attr, \"{}\")",
                st.name.to_upper_camel_case(),
                &st.name
            )
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_base_enum_def(
    w: &mut dyn Write,
    st: &Struct,
    parents: &[Rc<Struct>],
    config: &BaseTypeConfig,
) -> std::io::Result<()> {
    let base_name = st.name.to_upper_camel_case();
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub enum {} {{", base_name)?;
    indent(w, |w| {
        for st in parents
            .iter()
            .filter(|x| config.generate_base_type(&st.name, &x.name))
        {
            let child_name = st.name.to_upper_camel_case();
            writeln!(w, "{}(structs::{}),", child_name, child_name)?;
        }
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_base_enum_impl(
    w: &mut dyn Write,
    st: &Struct,
    parents: &[Rc<Struct>],
    config: &BaseTypeConfig,
) -> std::io::Result<()> {
    let base_name = st.name.to_upper_camel_case();
    writeln!(w, "impl {} {{", base_name)?;
    indent(w, |w| {
        writeln!(w, "pub(crate) fn write_with_name<W>(&self, writer: &mut xml::EventWriter<W>, name: &str) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(w, "match self {{")?;
            indent(w, |w| {
                for p in parents
                    .iter()
                    .filter(|x| config.generate_base_type(&st.name, &x.name))
                {
                    writeln!(
                        w,
                        "{}::{}(x) => x.write_with_name(writer, name, false, true),",
                        base_name,
                        p.name.to_upper_camel_case()
                    )?;
                }
                Ok(())
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)?;
        writeln!(w, "pub(crate) fn read<R>(reader: &mut xml::reader::EventReader<R>, attrs: &Vec<xml::attribute::OwnedAttribute>, parent_tag: &str) -> core::result::Result<Self, crate::ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "match crate::find_xsi_type(attrs)? {{")?;
            indent(w, |w| {
                for child in parents
                    .iter()
                    .filter(|x| config.generate_base_type(&st.name, &x.name))
                {
                    let child_name = child.name.to_upper_camel_case();
                    writeln!(
                        w,
                        "\"{}\" => Ok({}::{}(structs::{}::read(reader, attrs, parent_tag)?)),",
                        child.name,
                        base_name,
                        child_name,
                        child.name.to_upper_camel_case()
                    )?;
                }
                writeln!(w, "_ => return Err(crate::ReadError::UnknownXsiType),")
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;
        writeln!(w)
    })?;
    writeln!(w, "}}")
}

fn write_base_enums(dir: &Path, model: &Model, config: &BaseTypeConfig) -> Result<(), FatalError> {
    std::fs::create_dir(dir)?;

    let base_fields = model.base_fields();

    if base_fields.is_empty() {
        return Ok(());
    }

    // write the module file
    {
        let mut w = create(&dir.join("mod.rs"))?;
        for base in base_fields.iter() {
            writeln!(w, "mod {};", base.name.to_snake_case())?;
        }

        writeln!(w)?;

        for base in base_fields.iter() {
            writeln!(w, "pub use {}::*;", base.name.to_snake_case())?;
        }
    }

    for base in base_fields.iter() {
        let file = dir.join(format!("{}.rs", base.name.to_snake_case()));
        let mut w = create(&file)?;
        writeln!(w, "use super::super::*;")?;
        writeln!(w)?;
        let parents = model.sub_structs_of(base);
        write_base_enum_def(&mut w, base, parents.as_slice(), config)?;
        writeln!(w)?;
        write_base_enum_impl(&mut w, base, parents.as_slice(), config)?;
        writeln!(w)?;
    }

    Ok(())
}
