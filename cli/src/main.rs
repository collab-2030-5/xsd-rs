use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "xsd-cli", about = "XSD parser generator")]
struct Opt {
    /// Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let data = std::fs::read_to_string(opt.input).unwrap();
    let model = xsd_parser::parser::parse(&data).unwrap();
    println!("{:#?}", model);
}
