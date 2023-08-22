use openadr_xml::oadr::OadrPayload;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::{fs, str};
use xsd_api::*;

fn test_file<T: ReadXml + WriteXml + PartialEq + Debug>(name: &str) -> Result<(), Box<dyn Error>> {
    // the type as read from the 2030.5 sample data
    let first_read = T::read_file(Path::new("openadr/samplexml/input").join(name))?;

    // the data re-serialized using our XML writer
    let serialized = first_read.write_string(WriteConfig::default())?;

    // Write files to the output directory.  The python validator will test the files
    // in this location
    fs::write(
        Path::new("openadr/samplexml/output").join(name),
        serialized.clone(),
    )?;

    // the type as read from the 2030.5 sample data
    let second_read = T::read_str(serialized)?;

    assert_eq!(first_read, second_read);
    Ok(())
}

fn handle_file<T: ReadXml + WriteXml + PartialEq + Debug>(name: &str) {
    if let Err(err) = test_file::<T>(&name) {
        println!("*** ERROR processing file {}: {}", name, err);
    }
}

fn main() {
    let paths = fs::read_dir("openadr/samplexml/input").unwrap();

    for path in paths {
        let filename = path
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        handle_file::<OadrPayload>(&filename);
    }
}
