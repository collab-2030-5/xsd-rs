use std::path::PathBuf;

use heck::ToUpperCamelCase;
use structopt::StructOpt;
use xml_model::{Model, Struct};

#[derive(Debug, StructOpt)]
#[structopt(name = "xsd-cli", about = "XSD parser generator")]
struct Opt {
    /// Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
}

fn write_struct_fields(model: &Model, st: &Struct) {
    // write the base type fields
    if let Some(bt) = &st.base_type {
        if let Some(base) = model.structs.iter().find(|s| &s.name == bt) {
            write_struct_fields(model, base);

            for field in st.fields.iter() {
                println!();
                if let Some(comment) = &field.comment {
                    for line in comment.lines() {
                        println!("  /// {}", line);
                    }
                }
            }
        } else {
            println!(
                "    // Unable to find base type {} in struct {}",
                bt, st.name
            );
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let data = std::fs::read_to_string(opt.input).unwrap();
    let model = xsd_parser::transform(&data);

    for st in model.structs.iter() {
        println!("pub struct {} {{", st.name.to_upper_camel_case());
        write_struct_fields(&model, st);
        println!("}}");
        println!();
    }
}
