use std::collections::HashMap;
use std::io::{LineWriter, Write};
use std::path::Path;
use std::rc::Rc;

use indent_write::io::IndentWriter;

use xml_model::resolved::*;
use xml_model::{SimpleType, WrapperType};

use crate::{BaseTypeConfig, FatalError};

use heck::ToSnakeCase;

// pub(crate) mod base_enums;
pub(crate) mod bit_field;
pub(crate) mod named_array;
pub(crate) mod numeric_enum;
pub(crate) mod static_files;
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
            Self::Struct(x) => crate::gen::structs::write_struct(w, x),
            Self::Choice(_) => unimplemented!(),
            Self::Wrapper(x) => match x {
                WrapperType::Enum(x) => crate::gen::string_enums::write(w, x),
                WrapperType::EnumU8(x) => crate::gen::numeric_enum::write(w, x),
                WrapperType::NamedArray(x) => crate::gen::named_array::write(w, x),
                WrapperType::HexBitField(x) => crate::gen::bit_field::write(w, x),
            },
        }
    }
}

pub(crate) fn write_model(
    dir: &Path,
    model: &Model,
    _config: &BaseTypeConfig,
) -> Result<(), FatalError> {
    static_files::write(dir)?;

    let types_dir = dir.join("types");
    std::fs::create_dir(&types_dir)?;

    //
    let mut namespaces: HashMap<String, Vec<String>> = Default::default();

    for (id, any) in model.types.iter() {
        if let Some(gen_type) = GeneratedType::from(any) {
            let ns_dir = types_dir.join(&id.ns.to_snake_case());
            match namespaces.get_mut(&id.ns) {
                None => {
                    std::fs::create_dir(&ns_dir)?;
                    namespaces.insert(id.ns.clone(), vec![gen_type.name().to_snake_case()]);
                }
                Some(existing) => {
                    existing.push(gen_type.name().to_snake_case());
                }
            }

            let path = ns_dir.join(format!("{}.rs", gen_type.name().to_snake_case()));
            let mut writer = create(&path)?;
            gen_type.write(&mut writer)?;
        }
    }

    // use the extracted namespace info to generate all the mod.rs files
    {
        let mut w = create(&types_dir.join("mod.rs"))?;
        for ns in namespaces.keys() {
            writeln!(w, "pub mod {};", ns.to_snake_case())?;
        }
    }

    for (ns, types) in namespaces.iter() {
        let mut w = create(&types_dir.join(ns.to_snake_case()).join("mod.rs"))?;
        for typ in types {
            writeln!(&mut w, "mod {};", typ.to_snake_case())?;
        }

        writeln!(&mut w)?;
        writeln!(&mut w, "// re-export all the types in this namespace")?;
        writeln!(&mut w)?;

        for typ in types {
            writeln!(&mut w, "pub use {}::*;", typ.to_snake_case())?;
        }

        writeln!(&mut w)?;
        writeln!(&mut w, "// helpers specific to this namespace")?;
        write_add_schema_attr(&mut w, ns.as_ref())?;
    }

    Ok(())
}

fn create(path: &std::path::Path) -> Result<impl std::io::Write, FatalError> {
    let output = std::fs::File::create(path)?;
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
