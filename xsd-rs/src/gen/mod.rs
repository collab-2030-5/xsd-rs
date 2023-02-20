use std::collections::HashMap;
use std::io::{LineWriter, Write};
use std::path::Path;
use std::rc::Rc;

use indent_write::io::IndentWriter;

use xsd_model::resolved::*;
use xsd_model::{SimpleType, WrapperType};

use crate::FatalError;

use heck::ToSnakeCase;

pub(crate) mod bit_field;
pub(crate) mod choice;
pub(crate) mod fields;
pub(crate) mod named_array;
pub(crate) mod numeric_enum;
pub(crate) mod string_enums;
pub(crate) mod structs;
pub(crate) mod traits;

enum GeneratedType {
    Struct(Rc<Struct>),
    Choice(Rc<Choice>),
    Wrapper(WrapperType),
}

impl GeneratedType {
    fn name(&self) -> &str {
        match self {
            GeneratedType::Struct(x) => x.id.name.as_str(),
            GeneratedType::Wrapper(x) => x.name(),
            GeneratedType::Choice(x) => x.id.name.as_str(),
        }
    }

    fn from(any: &AnyType) -> Option<Self> {
        match any {
            AnyType::Simple(SimpleType::Primitive(_)) => None,
            AnyType::Simple(SimpleType::Wrapper(x)) => Some(GeneratedType::Wrapper(x.clone())),
            AnyType::Struct(x) => Some(GeneratedType::Struct(x.clone())),
            AnyType::Choice(x) => Some(GeneratedType::Choice(x.clone())),
        }
    }

    fn write(&self, w: &mut dyn Write) -> Result<(), FatalError> {
        match self {
            Self::Struct(x) => structs::write(w, x),
            Self::Choice(x) => choice::write(w, x),
            Self::Wrapper(x) => match x {
                WrapperType::Enum(x) => string_enums::write(w, x),
                WrapperType::EnumU8(_, x) => numeric_enum::write(w, x),
                WrapperType::NamedArray(_, x) => named_array::write(w, x),
                WrapperType::HexBitField(_, x) => bit_field::write(w, x),
            },
        }
    }
}

pub(crate) fn write_model(dir: &Path, model: Model) -> Result<(), FatalError> {
    let namespaces = split_into_namespaces(model);

    // use the extracted namespace info to generate all the parser files
    {
        let mut w = create(&dir.join("mod.rs"))?;
        for ns in namespaces.keys() {
            writeln!(w, "pub mod {};", ns.to_snake_case())?;
        }
    }

    for (ns, types) in namespaces.iter() {
        // create the directory for the namespace
        let ns_dir = dir.join(&ns.to_snake_case());

        std::fs::create_dir(&ns_dir)?;
        write_ns_mod_file(&ns_dir, ns, &types)?;

        for gen_type in types {
            let path = ns_dir.join(format!("{}.rs", gen_type.name().to_snake_case()));
            let mut writer = create(&path)?;
            gen_type.write(&mut writer)?;
        }
    }

    Ok(())
}

fn write_ns_mod_file(dir: &Path, ns: &str, types: &[GeneratedType]) -> Result<(), FatalError> {
    let mut w = create(&dir.join("mod.rs"))?;

    for typ in types {
        writeln!(w, "mod {};", typ.name().to_snake_case())?;
    }

    writeln!(w)?;

    writeln!(w, "// re-export all the types in this namespace")?;
    writeln!(w)?;
    for typ in types {
        writeln!(w, "pub use {}::*;", typ.name().to_snake_case())?;
    }

    // for now, we only need to write this if there's a struct present
    if types.iter().any(|x| matches!(x, GeneratedType::Struct(_))) {
        writeln!(w)?;
        writeln!(w, "// helpers specific to this namespace")?;
        write_add_schema_attr(&mut w, ns.as_ref())?;
    }

    Ok(())
}

fn split_into_namespaces(model: Model) -> HashMap<String, Vec<GeneratedType>> {
    let mut namespaces: HashMap<String, Vec<GeneratedType>> = Default::default();

    for (id, any) in model.simple_types.iter() {
        if let Some(gen_type) = GeneratedType::from(any) {
            match namespaces.get_mut(&id.ns) {
                None => {
                    namespaces.insert(id.ns.clone(), vec![gen_type]);
                }
                Some(existing) => {
                    existing.push(gen_type);
                }
            }
        }
    }

    for (id, any) in model.types.iter() {
        if let Some(gen_type) = GeneratedType::from(any) {
            match namespaces.get_mut(&id.ns) {
                None => {
                    namespaces.insert(id.ns.clone(), vec![gen_type]);
                }
                Some(existing) => {
                    existing.push(gen_type);
                }
            }
        }
    }

    namespaces
}

fn create(path: &Path) -> Result<impl std::io::Write, FatalError> {
    // let output = std::fs::File::open_(path)?;
    let output = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    tracing::info!("create: {}", path.display());
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

fn write_add_schema_attr(w: &mut dyn Write, ns: &str) -> std::io::Result<()> {
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
            writeln!(w, ".attr(\"xmlns\", \"{}\")", ns)
        })?;
        Ok(())
    })?;
    writeln!(w, "}}")
}
