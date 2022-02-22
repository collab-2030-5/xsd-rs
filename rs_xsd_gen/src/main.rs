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

fn get_attribute_fields(model: &Model, st: &Struct) -> Vec<Attribute> {
    let mut attr = Vec::new();

    if let Some(base_name) = &st.base_type {
        let base = model.structs.iter().find(|x| &x.name == base_name).unwrap();
        for field in get_attribute_fields(model, base) {
            attr.push(field);
        }
    }

    let attributes = st.fields.iter().filter_map(|x| match x.info {
        FieldTypeInfo::Attribute(att) => Some(Attribute {
            name: x.name.clone(),
            field_type: x.field_type.clone(),
            info: att,
        }),
        FieldTypeInfo::Element(_) => None,
    });

    for att in attributes {
        attr.push(att);
    }

    attr
}

enum AttributeTransform {
    String,
    Number,
}

impl AttributeTransform {
    fn write_value<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            AttributeTransform::String => Ok(()),
            AttributeTransform::Number => {
                writeln!(w, "let value = attr.to_string();")
            }
        }
    }

    fn transform_value(&self) -> &'static str {
        match self {
            AttributeTransform::String => "attr.as_str()",
            AttributeTransform::Number => "value.as_str()",
        }
    }
}

fn get_transform(model: &Model, attr_type: &str) -> AttributeTransform {
    match attr_type {
        "xs:anyURI" => AttributeTransform::String,
        _ => {
            // try to resolve as a simple type
            match model.simple_types.get(attr_type) {
                None => {
                    panic!("unknown attribute type: {}", attr_type)
                }
                Some(st) => match st {
                    SimpleType::Alias(x) => get_transform(model, x),
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

fn write_attribute<W>(w: &mut W, model: &Model, att: &Attribute) -> std::io::Result<()>
where
    W: Write,
{
    match att.info {
        AttributeType::Single => {
            writeln!(w, "// TODO write attribute {}", &att.name)?;
        }
        AttributeType::Option => {
            let transform = get_transform(model, &att.field_type);
            writeln!(
                w,
                "if let Some(attr) = &self.{} {{",
                &att.name.to_snake_case()
            )?;
            indent(w, |w| {
                transform.write_value(w)?;
                writeln!(
                    w,
                    "start.push_attribute((\"{}\", {}));",
                    att.name,
                    transform.transform_value()
                )?;
                Ok(())
            })?;
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

fn bytes(s: &str) -> String {
    format!("b\"{}\"", s)
}

fn write_model<W>(mut w: W, model: &Model) -> std::io::Result<()>
where
    W: Write,
{
    for st in &model.structs {
        writeln!(w, "pub struct {} {{", st.name.to_upper_camel_case())?;
        indent(&mut w, |w| write_struct_fields(w, model, st))?;
        writeln!(w, "}}")?;
    }

    // write the serializers
    for st in &model.structs {
        // collect all the attribute fields
        let attributes = get_attribute_fields(model, st);

        writeln!(w, "impl {} {{", st.name.to_upper_camel_case())?;
        indent(&mut w, |w| {
            writeln!(w, "pub fn write<W>(&self, writer: &mut quick_xml::Writer<W>) -> Result<(), quick_xml::Error> where W: std::io::Write {{")?;
            indent(w, |w| {
                let attr_start_mod = if attributes.is_empty() { " " } else { " mut " };

                writeln!(
                    w,
                    "let{}start = quick_xml::events::BytesStart::borrowed_name({});",
                    attr_start_mod,
                    bytes(&st.name)
                )?;

                // write the attributes
                for att in &attributes {
                    write_attribute(w, model, att)?;
                }

                writeln!(
                    w,
                    "writer.write_event(quick_xml::events::Event::Start(start))?;"
                )?;

                // todo write the fields

                writeln!(w, "writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::borrowed({})))?;", bytes(&st.name))?;
                writeln!(w, "Ok(())")
            })?;
            writeln!(w, "}}")
        })?;
        writeln!(w, "}}")?;
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
