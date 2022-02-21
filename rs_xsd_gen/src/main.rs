use structopt::StructOpt;

use heck::{ToSnakeCase, ToUpperCamelCase};
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;
use xml_model::{Model, SimpleType, Struct};

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
        Type::Struct(x) => x,
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
    writeln!(writer, "  // --- these fields come from {} --- ", st.name)?;
    writeln!(writer)?;
    for field in &st.fields {
        let rust_type = get_rust_type(model, resolve(model, &field.field_type));
        if let Some(x) = &field.comment {
            for line in x.lines() {
                writeln!(writer, "  // {}", line)?;
            }
        }
        writeln!(writer, "  {}: {},", field.name.to_snake_case(), rust_type)?;
    }
    writeln!(writer)
}

fn write_model<W>(mut writer: W, model: &Model) -> std::io::Result<()>
where
    W: Write,
{
    for st in &model.structs {
        writeln!(writer, "pub struct {} {{", st.name.to_upper_camel_case())?;
        write_struct_fields(&mut writer, model, st)?;
        writeln!(writer, "}}")?;
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
