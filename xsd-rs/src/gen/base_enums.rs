use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use xml_model::resolved::*;

use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::gen::{create, indent};
use crate::*;

pub(crate) fn write(dir: &Path, model: &Model, config: &BaseTypeConfig) -> Result<(), FatalError> {
    let base_fields = model.base_fields();

    // write the module file
    {
        let mut w = create(&dir.join("parser"))?;
        for base in base_fields.iter() {
            writeln!(w, "mod {};", base.id.name.to_snake_case())?;
        }

        writeln!(w)?;

        for base in base_fields.iter() {
            writeln!(w, "pub use {}::*;", base.id.name.to_snake_case())?;
        }
    }

    for base in base_fields.iter() {
        let file = dir.join(format!("{}.rs", base.id.name.to_snake_case()));
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

fn write_base_enum_def(
    w: &mut dyn Write,
    st: &Struct,
    parents: &[Rc<Struct>],
    config: &BaseTypeConfig,
) -> std::io::Result<()> {
    let base_name = st.id.name.to_upper_camel_case();
    writeln!(w, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(w, "pub enum {} {{", base_name)?;
    indent(w, |w| {
        for st in parents
            .iter()
            .filter(|x| config.generate_base_type(&st.id.name, &x.id.name))
        {
            let child_name = st.id.name.to_upper_camel_case();
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
    let base_name = st.id.name.to_upper_camel_case();
    writeln!(w, "impl {} {{", base_name)?;
    indent(w, |w| {
        writeln!(w, "pub(crate) fn write_with_name<W>(&self, writer: &mut xml::EventWriter<W>, name: &str) -> core::result::Result<(), xml::writer::Error> where W: std::io::Write {{")?;
        indent(w, |w| {
            writeln!(w, "match self {{")?;
            indent(w, |w| {
                for p in parents
                    .iter()
                    .filter(|x| config.generate_base_type(&st.id.name, &x.id.name))
                {
                    writeln!(
                        w,
                        "{}::{}(x) => x.write_with_name(writer, name, false, true),",
                        base_name,
                        p.id.name.to_upper_camel_case()
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
                    .filter(|x| config.generate_base_type(&st.id.name, &x.id.name))
                {
                    let child_name = child.id.name.to_upper_camel_case();
                    writeln!(
                        w,
                        "\"{}\" => Ok({}::{}(structs::{}::read(reader, attrs, parent_tag)?)),",
                        child.id,
                        base_name,
                        child_name,
                        child.id.name.to_upper_camel_case()
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
