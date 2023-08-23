use openadr_xml::oadr::OadrPayload;
use std::error::Error;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
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
        tracing::error!("Failed processing file {}: {}", name, err);
    }
}

fn validate_w_xsd(path: &str) {
    // Run python XSD validator
    let stdout = Command::new("python")
        .arg("openadr/validate.py")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .for_each(|line| println!("{}", line.unwrap()));
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Remove any existing output files and recreate the output directory
    let output_path_name = "openadr/samplexml/output";
    fs::remove_dir_all(Path::new(output_path_name)).unwrap();
    fs::create_dir(Path::new(output_path_name)).unwrap();

    // XSD validate the input fiels
    let input_path_name = "openadr/samplexml/input";
    let path = fs::read_dir(input_path_name).unwrap();

    tracing::info!("Performing XSD validation on input files");
    validate_w_xsd(&input_path_name);

    // Process the input files with the generated code
    // Write new files to the output directory
    tracing::info!("Processing input files");
    for file in path {
        let filename = file
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        handle_file::<OadrPayload>(&filename);
    }

    // XSD validate the output files
    tracing::info!("Performing XSD validation on output files");
    validate_w_xsd(&output_path_name);
}
