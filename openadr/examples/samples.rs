use openadr_xml::oadr::OadrPayload;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use xsd_api::*;

fn test_file<T: ReadXml + WriteXml + PartialEq + Debug>(name: &str) -> Result<(), Box<dyn Error>> {
    // the type as read from the 2030.5 sample data
    let first_read = T::read_file(Path::new("openadr/samplexml").join(name))?;

    // the data re-serialized using our XML writer
    let serialized = first_read.write_string(WriteConfig::default())?;

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
    handle_file::<OadrPayload>("create_party_registration.xml");
}
