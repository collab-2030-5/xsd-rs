use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) mod parse;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "xsd-transform",
    about = "Transforms subset of XSD into a simplified JSON model"
)]
struct Opt {
    /// xsd input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
    /// json output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let xml = std::fs::read_to_string(opt.input).unwrap();
    let model = crate::parse::transform(&xml);

    let file = std::fs::File::create(opt.output).unwrap();
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &model).unwrap();
}
