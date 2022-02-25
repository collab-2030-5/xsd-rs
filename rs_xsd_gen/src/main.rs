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

fn resolve_basic_type(name: &str) -> Option<BasicType> {
    match name {
        "xs:boolean" => Some(BasicType::Boolean),
        "xs:anyURI" => Some(BasicType::AnyUri),
        "xs:string" => Some(BasicType::String),
        _ => None,
    }
}

fn resolve_type(model: &Model, name: &str) -> Type {
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

fn write_struct_fields(writer: &mut dyn Write, model: &Model, st: &Struct) -> std::io::Result<()> {
    if let Some(bt) = &st.base_type {
        match model.structs.iter().find(|st| &st.name == bt) {
            None => panic!("cannot resolve base type {} in {}", bt, st.name),
            Some(st) => {
                write_struct_fields(writer, model, st)?;
            }
        }
    }

    writeln!(writer)?;
    writeln!(writer, "// --- these fields come from {} ---", st.name)?;
    writeln!(writer)?;
    for field in &st.fields {
        let rust_type = get_rust_type(model, resolve_type(model, &field.field_type));

        write_comment(writer, &field.comment)?;

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
    Number,
}

impl AttributeTransform {
    fn transform_to_string(&self, name: &str) -> String {
        match self {
            AttributeTransform::Number => format!("{}.to_string()", name),
        }
    }
}

fn get_attr_transform(model: &Model, attr_type: &str) -> Option<AttributeTransform> {
    match attr_type {
        "xs:anyURI" => None,
        _ => {
            // try to resolve as a simple type
            match model.simple_types.get(attr_type) {
                None => {
                    panic!("unknown attribute type: {}", attr_type)
                }
                Some(st) => match st {
                    SimpleType::Alias(x) => get_attr_transform(model, x),
                    SimpleType::HexByte => Some(AttributeTransform::Number),
                    SimpleType::HexBytes(_) => None,
                    SimpleType::Enum(_) => unimplemented!(),
                    SimpleType::String(_) => None,
                    SimpleType::I8(_) => Some(AttributeTransform::Number),
                    SimpleType::U8(_) => Some(AttributeTransform::Number),
                    SimpleType::I16(_) => Some(AttributeTransform::Number),
                    SimpleType::U16(_) => Some(AttributeTransform::Number),
                    SimpleType::I32(_) => Some(AttributeTransform::Number),
                    SimpleType::U32(_) => Some(AttributeTransform::Number),
                    SimpleType::I64(_) => Some(AttributeTransform::Number),
                    SimpleType::U64(_) => Some(AttributeTransform::Number),
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
                writeln!(w, "writer.write({}.as_str())?;", rust_name)
            }
            ElementTransform::Number => {
                writeln!(w, "let value = {}.to_string();", rust_name)?;
                writeln!(w, "writer.write(value.as_str())?;")
            }
            ElementTransform::HexBytes => {
                writeln!(
                    w,
                    "let hex : String = {}.iter().map(|x| format!(\"{{:02x}}\", x)).collect();",
                    rust_name
                )?;
                writeln!(w, "writer.write(hex.as_str())?;")
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
    let name = get_rust_field_name(&att.name);
    let self_name = format!("self.{}", &name);
    let transform = get_attr_transform(model, &att.field_type);

    match att.info {
        AttributeType::Single => {
            if let Some(tx) = &transform {
                writeln!(w, "let {} = {};", &name, tx.transform_to_string(&self_name))?;
                writeln!(
                    w,
                    "let start = start.attr(\"{}\", {}.as_str());",
                    att.name, &name
                )?;
            } else {
                writeln!(
                    w,
                    "let start = start.attr(\"{}\", {}.as_str());",
                    att.name, &self_name
                )?;
            }
        }
        AttributeType::Option => {
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
                    writeln!(w, "start.attr(\"{}\", attr.as_str())", att.name)
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

fn write_lines(w: &mut dyn Write, s: &str) -> std::io::Result<()> {
    for line in s.lines() {
        writeln!(w, "{}", line)?;
    }
    Ok(())
}

fn write_add_schema_attr(w: &mut dyn Write, model: &Model) -> std::io::Result<()> {
    let target_ns = model.target_ns.as_ref().expect("requires target namespace");

    writeln!(
        w,
        "fn add_schema_attr(start: events::StartElementBuilder) -> events::StartElementBuilder {{"
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

fn write_serializers(w: &mut dyn Write, st: &Struct, model: &Model) -> std::io::Result<()> {
    // collect all the attribute fields
    let (attributes, elements) = split_fields(model, st);

    writeln!(w, "impl {} {{", st.name.to_upper_camel_case())?;
    indent(w, |w| {
        if !elements.is_empty() {
            writeln!(w, "fn write_elem<W>(&self, writer: &mut EventWriter<W>) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
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

        writeln!(w, "fn write_with_name<W>(&self, writer: &mut EventWriter<W>, name: &str, top_level: bool) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(w, "let start = if top_level {{ add_schema_attr(events::XmlEvent::start_element(name)) }} else {{ events::XmlEvent::start_element(name) }};")?;

            if !attributes.is_empty() {
                writeln!(w, "// ---- start attributes ----")?;
                for att in &attributes {
                    write_attribute(w, model, att)?;
                }
                writeln!(w, "// ---- end attributes ----")?;
            }

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
        "impl WriteToXml for {} {{",
        st.name.to_upper_camel_case()
    )?;
    indent(w, |w| {
        writeln!(w, "fn write_to_xml<W>(&self, writer: &mut W) -> core::result::Result<(), WriteError> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(
                w,
                "let mut writer = EmitterConfig::new().create_writer(writer);"
            )?;
            writeln!(
                w,
                "self.write_with_name(&mut writer, \"{}\", true)?;",
                st.name
            )?;
            writeln!(w, "Ok(())")
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn write_model(w: &mut dyn Write, model: &Model) -> std::io::Result<()> {
    // write all the snippets
    write_lines(w, include_str!("../snippets/use_statements.rs"))?;
    writeln!(w)?;
    write_lines(w, include_str!("../snippets/traits.rs"))?;
    writeln!(w)?;

    write_add_schema_attr(w, model)?;
    writeln!(w)?;

    // write the struct definitions
    for st in &model.structs {
        write_comment(w, &st.comment)?;
        writeln!(w, "pub struct {} {{", st.name.to_upper_camel_case())?;
        indent(w, |w| write_struct_fields(w, model, st))?;
        writeln!(w, "}}")?;
    }

    // write the serialization impl and trait
    for st in &model.structs {
        write_serializers(w, st, model)?;
        writeln!(w)?;
        write_deserializers(w, st, model)?;
    }

    writeln!(w)?;

    write_lines(w, include_str!("../snippets/error.rs"))?;
    writeln!(w)?;
    write_lines(w, include_str!("../snippets/read_helpers.rs"))?;

    Ok(())
}

fn write_struct_cells(w: &mut dyn Write, st: &Struct, model: &Model) -> std::io::Result<()> {
    // do this recursively depth-first
    if let Some(bt) = &st.base_type {
        match model.structs.iter().find(|st| &st.name == bt) {
            None => panic!("cannot resolve base type {} in {}", bt, st.name),
            Some(st) => {
                write_struct_cells(w, st, model)?;
            }
        }
    }

    for field in &st.fields {
        let rust_type = get_rust_type(model, resolve_type(model, &field.field_type));
        let cell_type = match &field.info {
            FieldTypeInfo::Attribute(x) => match x {
                AttributeType::Single => format!("SetOnce<{}>", rust_type),
                AttributeType::Option => format!("SetOnce<{}>", rust_type),
            },
            FieldTypeInfo::Element(x) => match x {
                ElementType::Single => format!("SetOnce<{}>", rust_type),
                ElementType::Array => format!("Vec<{}>", rust_type),
                ElementType::Option => format!("SetOnce<{}>", rust_type),
                ElementType::Error(x) => panic!("{}", x),
            },
        };

        writeln!(
            w,
            "let {} : {} = Default::default();",
            get_rust_field_name(&field.name),
            cell_type
        )?;
    }

    Ok(())
}

fn write_struct_initializer(w: &mut dyn Write, st: &Struct, model: &Model) -> std::io::Result<()> {
    // do this recursively depth-first
    if let Some(bt) = &st.base_type {
        match model.structs.iter().find(|st| &st.name == bt) {
            None => panic!("cannot resolve base type {} in {}", bt, st.name),
            Some(st) => {
                write_struct_initializer(w, st, model)?;
            }
        }
    }

    for field in &st.fields {
        let rust_var = get_rust_field_name(&field.name);
        match &field.info {
            FieldTypeInfo::Attribute(x) => match x {
                AttributeType::Single => writeln!(w, "{} : {}.expect()?,", &rust_var, &rust_var),
                AttributeType::Option => writeln!(w, "{} : {}.get(),", &rust_var, &rust_var),
            },
            FieldTypeInfo::Element(x) => match x {
                ElementType::Single => writeln!(w, "{} : {}.expect()?,", &rust_var, &rust_var),
                ElementType::Array => writeln!(w, "{},", &rust_var),
                ElementType::Option => writeln!(w, "{} : {}.get(),", &rust_var, &rust_var),
                ElementType::Error(x) => panic!("{}", x),
            },
        }?;
    }

    Ok(())
}

fn write_deserializers(w: &mut dyn Write, st: &Struct, model: &Model) -> std::io::Result<()> {
    // categorize the fields
    //let (attr, elem) = split_fields(model, st);
    writeln!(
        w,
        "impl ReadFromXml for {} {{",
        st.name.to_upper_camel_case()
    )?;
    indent(w, |w| {
        writeln!(w, "fn read_from_xml<R>(_reader: &mut R) -> core::result::Result<Self, ReadError> where R: std::io::Read {{")?;
        indent(w, |w| {
            writeln!(w, "// one variable for each attribute and element")?;
            write_struct_cells(w, st, model)?;
            writeln!(w, "// TODO - parse the values!")?;
            writeln!(w)?;
            writeln!(w, "Ok({} {{", st.name.to_upper_camel_case())?;
            indent(w, |w| write_struct_initializer(w, st, model))?;
            writeln!(w, "}})")
        })?;
        writeln!(w, "}}")?;
        Ok(())
    })?;
    writeln!(w, "}}")
}

fn main() {
    let opt = Opt::from_args();
    let input = std::fs::read_to_string(opt.input).unwrap();
    let model: Model = serde_json::from_str(&input).unwrap();

    let output = std::fs::File::create(opt.output).unwrap();
    let mut writer = LineWriter::new(output);

    write_model(&mut writer, &model).unwrap();
}
