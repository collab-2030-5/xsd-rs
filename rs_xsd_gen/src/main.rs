use structopt::StructOpt;

use heck::{ToSnakeCase, ToUpperCamelCase};
use indent_write::io::IndentWriter;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;
use xml_model::{AttributeType, ElementType, FieldTypeInfo, Model, SimpleType, Struct};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rs-xsd-gen",
    about = "Reads model of xml from json and emits rust bindings"
)]
struct Opt {
    /// json input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
    // rust output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

enum BasicType {
    Boolean,
    String,
    AnyUri,
}

enum Type {
    Basic(BasicType),
    Simple(SimpleType),
    Struct(String),
}

fn indent<W, F>(w: &mut W, f: F) -> std::io::Result<()>
where
    W: Write,
    F: Fn(&mut IndentWriter<&mut W>) -> std::io::Result<()>,
{
    let mut w = IndentWriter::new("    ", w);
    f(&mut w)
}

fn resolve_basic_type(name: &str) -> Option<BasicType> {
    match name {
        "xs:boolean" => Some(BasicType::Boolean),
        "xs:anyURI" => Some(BasicType::AnyUri),
        "xs:string" => Some(BasicType::String),
        _ => None,
    }
}

fn resolve(model: &Model, name: &str) -> Type {
    if let Some(basic) = resolve_basic_type(name) {
        return Type::Basic(basic);
    }

    if let Some(x) = model.simple_types.get(name) {
        return Type::Simple(x.clone());
    }

    match model.structs.iter().find(|st| st.name == name) {
        None => {
            panic!("No match for referenced type {}", name);
        }
        Some(st) => Type::Struct(st.name.clone()),
    }
}

fn resolve_rust_simple_type(model: &Model, x: &SimpleType) -> String {
    match x {
        SimpleType::Alias(x) => {
            let alias = model.simple_types.get(x).unwrap();
            resolve_rust_simple_type(model, alias)
        }
        SimpleType::HexByte => "u8".to_string(),
        SimpleType::HexBytes(_) => "Vec<u8>".to_string(),
        SimpleType::Enum(_) => unimplemented!(),
        SimpleType::String(_) => "String".to_string(),
        SimpleType::I8(_) => "i8".to_string(),
        SimpleType::U8(_) => "u8".to_string(),
        SimpleType::I16(_) => "i16".to_string(),
        SimpleType::U16(_) => "u16".to_string(),
        SimpleType::I32(_) => "i32".to_string(),
        SimpleType::U32(_) => "u32".to_string(),
        SimpleType::I64(_) => "i64".to_string(),
        SimpleType::U64(_) => "u64".to_string(),
    }
}

fn get_rust_type(model: &Model, t: Type) -> String {
    match t {
        Type::Basic(x) => match x {
            BasicType::Boolean => "bool".to_string(),
            BasicType::String => "String".to_string(),
            BasicType::AnyUri => "String".to_string(),
        },
        Type::Simple(x) => resolve_rust_simple_type(model, &x),
        Type::Struct(x) => x.to_upper_camel_case(),
    }
}

fn get_rust_field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    match snake.as_str() {
        // have to rename reserved identifiers
        "type" => "typ".to_string(),
        _ => snake,
    }
}

fn write_struct_fields<W>(writer: &mut W, model: &Model, st: &Struct) -> std::io::Result<()>
where
    W: Write,
{
    if let Some(bt) = &st.base_type {
        match model.structs.iter().find(|st| &st.name == bt) {
            None => panic!("cannot resolve base type {} in {}", bt, st.name),
            Some(st) => {
                write_struct_fields(writer, model, st)?;
            }
        }
    }

    writeln!(writer)?;
    writeln!(writer, "// --- these fields come from {} --- ", st.name)?;
    writeln!(writer)?;
    for field in &st.fields {
        let rust_type = get_rust_type(model, resolve(model, &field.field_type));
        if let Some(x) = &field.comment {
            for line in x.lines() {
                writeln!(writer, "// {}", line)?;
            }
        }
        let rust_type = match &field.info {
            FieldTypeInfo::Attribute(x) => match x {
                AttributeType::Single => rust_type,
                AttributeType::Option => format!("Option<{}>", rust_type),
            },
            FieldTypeInfo::Element(x) => match x {
                ElementType::Single => rust_type,
                ElementType::Array => format!("Vec<{}>", rust_type),
                ElementType::Option => format!("Option<{}>", rust_type),
                ElementType::Error(x) => {
                    panic!("{}", x);
                }
            },
        };

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
    field_type: String,
    info: AttributeType,
}

struct Element {
    name: String,
    field_type: String,
    info: ElementType,
}

fn split_fields(model: &Model, st: &Struct) -> (Vec<Attribute>, Vec<Element>) {
    let mut attrs = Vec::new();
    let mut elems = Vec::new();

    if let Some(base_name) = &st.base_type {
        let base = model.structs.iter().find(|x| &x.name == base_name).unwrap();
        let (attr, elem) = split_fields(model, base);
        for field in attr {
            attrs.push(field);
        }
        for field in elem {
            elems.push(field);
        }
    }

    for field in &st.fields {
        match &field.info {
            FieldTypeInfo::Attribute(x) => {
                let x = Attribute {
                    name: field.name.clone(),
                    field_type: field.field_type.clone(),
                    info: *x,
                };
                attrs.push(x);
            }
            FieldTypeInfo::Element(x) => {
                let x = Element {
                    name: field.name.clone(),
                    field_type: field.field_type.clone(),
                    info: x.clone(),
                };
                elems.push(x);
            }
        }
    }

    (attrs, elems)
}

enum AttributeTransform {
    String,
    Number,
}

impl AttributeTransform {
    fn write_value<W>(&self, name: &str, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            AttributeTransform::String => Ok(()),
            AttributeTransform::Number => {
                writeln!(w, "let value = {}.to_string();", name)
            }
        }
    }

    fn transform_value(&self, name: &str) -> String {
        match self {
            AttributeTransform::String => format!("{}.as_str()", name),
            AttributeTransform::Number => "value.as_str()".to_string(),
        }
    }
}

fn get_attr_transform(model: &Model, attr_type: &str) -> AttributeTransform {
    match attr_type {
        "xs:anyURI" => AttributeTransform::String,
        _ => {
            // try to resolve as a simple type
            match model.simple_types.get(attr_type) {
                None => {
                    panic!("unknown attribute type: {}", attr_type)
                }
                Some(st) => match st {
                    SimpleType::Alias(x) => get_attr_transform(model, x),
                    SimpleType::HexByte => AttributeTransform::Number,
                    SimpleType::HexBytes(_) => AttributeTransform::String,
                    SimpleType::Enum(_) => unimplemented!(),
                    SimpleType::String(_) => AttributeTransform::String,
                    SimpleType::I8(_) => AttributeTransform::Number,
                    SimpleType::U8(_) => AttributeTransform::Number,
                    SimpleType::I16(_) => AttributeTransform::Number,
                    SimpleType::U16(_) => AttributeTransform::Number,
                    SimpleType::I32(_) => AttributeTransform::Number,
                    SimpleType::U32(_) => AttributeTransform::Number,
                    SimpleType::I64(_) => AttributeTransform::Number,
                    SimpleType::U64(_) => AttributeTransform::Number,
                },
            }
        }
    }
}

enum ElementTransform {
    Struct,
    Number,
    String,
    HexBytes,
}

impl ElementTransform {
    fn write_value<W>(&self, w: &mut W, rust_name: &str, xsd_name: &str) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            ElementTransform::Struct => {
                writeln!(
                    w,
                    "{}.write_with_name(writer, \"{}\", false)?;",
                    rust_name, xsd_name
                )
            }
            ElementTransform::String => {
                writeln!(w, "writer.write({}.as_bytes())?;", rust_name)
            }
            ElementTransform::Number => {
                writeln!(w, "let value = {}.to_string();", rust_name)?;
                writeln!(w, "writer.write(value.as_bytes())?;")
            }
            ElementTransform::HexBytes => {
                writeln!(
                    w,
                    "let hex : String = {}.iter().map(|x| format!(\"{{:02x}}\", x)).collect();",
                    rust_name
                )?;
                writeln!(w, "writer.write(hex.as_bytes())?;")
            }
        }
    }
}

fn get_simple_type_transform(model: &Model, st: &SimpleType) -> ElementTransform {
    match st {
        SimpleType::Alias(x) => {
            get_simple_type_transform(model, model.simple_types.get(x).unwrap())
        }
        SimpleType::HexByte => ElementTransform::Number,
        SimpleType::HexBytes(_) => ElementTransform::HexBytes,
        SimpleType::Enum(_) => unimplemented!(),
        SimpleType::String(_) => ElementTransform::String,
        SimpleType::I8(_) => ElementTransform::Number,
        SimpleType::U8(_) => ElementTransform::Number,
        SimpleType::I16(_) => ElementTransform::Number,
        SimpleType::U16(_) => ElementTransform::Number,
        SimpleType::I32(_) => ElementTransform::Number,
        SimpleType::U32(_) => ElementTransform::Number,
        SimpleType::I64(_) => ElementTransform::Number,
        SimpleType::U64(_) => ElementTransform::Number,
    }
}

fn get_elem_transform(model: &Model, elem_type: &str) -> ElementTransform {
    match elem_type {
        "xs:string" => ElementTransform::String,
        "xs:anyURI" => ElementTransform::String,
        "xs:boolean" => ElementTransform::Number,
        _ => {
            // is it a struct?
            match model.structs.iter().find(|st| st.name == elem_type) {
                None => match model.simple_types.get(elem_type) {
                    None => {
                        panic!("unknown element type: {}", elem_type)
                    }
                    Some(x) => get_simple_type_transform(model, x),
                },
                Some(_) => ElementTransform::Struct,
            }
        }
    }
}

fn write_element<W>(w: &mut W, model: &Model, elem: &Element) -> std::io::Result<()>
where
    W: Write,
{
    let transform = get_elem_transform(model, &elem.field_type);

    match &elem.info {
        ElementType::Single => {
            let name = format!("self.{}", get_rust_field_name(&elem.name));
            transform.write_value(w, &name, &elem.name)?;
        }
        ElementType::Array => {
            writeln!(
                w,
                "for item in &self.{} {{",
                get_rust_field_name(&elem.name)
            )?;
            indent(w, |w| transform.write_value(w, "item", &elem.name))?;
            writeln!(w, "}}")?;
        }
        ElementType::Option => {
            writeln!(
                w,
                "if let Some(elem) = &self.{} {{",
                elem.name.to_snake_case()
            )?;
            indent(w, |w| transform.write_value(w, "elem", &elem.name))?;
            writeln!(w, "}}")?;
        }
        ElementType::Error(s) => {
            panic!("error: {}", s)
        }
    }

    Ok(())
}

fn write_attribute<W>(w: &mut W, model: &Model, att: &Attribute) -> std::io::Result<()>
where
    W: Write,
{
    match att.info {
        AttributeType::Single => {
            let transform = get_attr_transform(model, &att.field_type);
            writeln!(w, "{{")?;
            indent(w, |w| {
                let name = format!("self.{}", att.name.to_snake_case());
                transform.write_value(&name, w)?;
                writeln!(
                    w,
                    "start.push_attribute((\"{}\", {}));",
                    att.name,
                    transform.transform_value(&name)
                )?;
                Ok(())
            })?;
            writeln!(w, "}}")?;
        }
        AttributeType::Option => {
            let transform = get_attr_transform(model, &att.field_type);
            writeln!(
                w,
                "if let Some(attr) = &self.{} {{",
                &att.name.to_snake_case()
            )?;
            indent(w, |w| {
                transform.write_value("attr", w)?;
                writeln!(
                    w,
                    "start.push_attribute((\"{}\", {}));",
                    att.name,
                    transform.transform_value("attr")
                )?;
                Ok(())
            })?;
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

fn write_model<W>(mut w: W, model: &Model) -> std::io::Result<()>
where
    W: Write,
{
    let target_ns = model.target_ns.as_ref().expect("requires target namespace");

    writeln!(
        w,
        "fn add_xsi_attr(start: &mut quick_xml::events::BytesStart) {{"
    )?;
    indent(&mut w, |w| {
        writeln!(
            w,
            "start.push_attribute((\"xmlns:xsi\", \"http://www.w3.org/2001/XMLSchema-instance\"));"
        )?;
        writeln!(
            w,
            "start.push_attribute((\"xmlns:xsd\", \"http://www.w3.org/2001/XMLSchema\"));"
        )
    })?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(
        w,
        "fn add_target_ns_attr(start: &mut quick_xml::events::BytesStart) {{"
    )?;
    indent(&mut w, |w| {
        writeln!(
            w,
            "start.push_attribute((\"xmlns\", \"{}\"));",
            target_ns.uri
        )
    })?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    for st in &model.structs {
        writeln!(w, "pub struct {} {{", st.name.to_upper_camel_case())?;
        indent(&mut w, |w| write_struct_fields(w, model, st))?;
        writeln!(w, "}}")?;
    }

    // write the serializers
    for st in &model.structs {
        // collect all the attribute fields
        let (attributes, elements) = split_fields(model, st);

        writeln!(w, "impl {} {{", st.name.to_upper_camel_case())?;
        indent(&mut w, |w| {
            // write the attribute serializer
            if !attributes.is_empty() {
                writeln!(
                    w,
                    "fn write_attr(&self, start: &mut quick_xml::events::BytesStart) {{"
                )?;
                indent(w, |w| {
                    // write the attributes
                    for att in &attributes {
                        write_attribute(w, model, att)?;
                    }
                    Ok(())
                })?;
                writeln!(w, "}}")?;

                writeln!(w)?;
            }

            if !elements.is_empty() {
                writeln!(w, "fn write_elem<W>(&self, writer: &mut quick_xml::Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {{")?;
                indent(w, |w| {
                    // write the elements
                    for elem in &elements {
                        write_element(w, model, elem)?;
                    }
                    writeln!(w, "Ok(())")
                })?;
                writeln!(w, "}}")?;

                writeln!(w)?;
            }

            writeln!(w, "pub fn write<W>(&self, writer: &mut W) -> Result<(), WriteError> where W: std::io::Write {{")?;
            indent(w, |w| {
                writeln!(w, "let mut writer = quick_xml::Writer::new(writer);")?;
                writeln!(
                    w,
                    "self.write_with_name(&mut writer, \"{}\", true)?;",
                    st.name
                )?;
                writeln!(w, "Ok(())")
            })?;
            writeln!(w, "}}")?;
            writeln!(w)?;

            writeln!(w, "fn write_with_name<W>(&self, writer: &mut quick_xml::Writer<W>, name: &str, top_level: bool) -> Result<(), quick_xml::Error> where W: std::io::Write {{")?;
            indent(w, |w| {
                writeln!(w, "let mut start = quick_xml::events::BytesStart::borrowed_name(name.as_bytes());")?;

                writeln!(w, "if top_level {{")?;
                indent(w, |w| writeln!(w, "add_xsi_attr(&mut start);"))?;
                writeln!(w, "}}")?;

                if !attributes.is_empty() {
                    writeln!(w, "self.write_attr(&mut start);")?;
                }

                writeln!(w, "if top_level {{")?;
                indent(w, |w| writeln!(w, "add_target_ns_attr(&mut start);"))?;
                writeln!(w, "}}")?;

                writeln!(
                    w,
                    "writer.write_event(quick_xml::events::Event::Start(start))?;"
                )?;

                if !elements.is_empty() {
                    writeln!(w, "self.write_elem(writer)?;")?;
                }

                writeln!(w, "writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::borrowed(name.as_bytes())))?;")?;
                writeln!(w, "Ok(())")
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;
    }

    writeln!(w)?;

    let error = include_str!("../snippets/error.rs");

    for line in error.lines() {
        writeln!(w, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let input = std::fs::read_to_string(opt.input).unwrap();
    let model: Model = serde_json::from_str(&input).unwrap();

    let output = std::fs::File::create(opt.output).unwrap();
    let writer = LineWriter::new(output);

    write_model(writer, &model).unwrap();
}
