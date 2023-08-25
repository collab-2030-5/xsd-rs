pub(crate) mod gen;
pub(crate) mod options;

use std::path::Path;

use crate::options::{Commands, GenerateOptions, InspectOptions};
use gen::traits::RustType;
use xsd_model::unresolved::UnresolvedModel;

type FatalError = Box<dyn std::error::Error>;

fn main() -> Result<(), FatalError> {
    let options = options::Options::get();

    tracing_subscriber::fmt()
        .with_max_level(options.log_level())
        .with_target(false)
        .init();

    match options.command {
        Commands::Generate(x) => generate(&x),
        Commands::Inspect(x) => inspect(&x),
    }
}

fn inspect(options: &InspectOptions) -> Result<(), FatalError> {
    tracing::info!("reading input file");
    let xsd = std::fs::read_to_string(&options.input)?;
    tracing::info!("parsing XSD");
    let xsd = xsd_model::parse_xsd(&xsd);
    tracing::info!("Read: {:#?}", xsd);
    Ok(())
}

fn generate(options: &GenerateOptions) -> Result<(), FatalError> {
    let config = {
        let span = tracing::info_span!("init");
        let _guard = span.enter();
        tracing::info!("reading configuration");
        let config: xsd_model::config::Config =
            serde_json::from_reader(std::fs::File::open(&options.config)?)?;
        config
    };

    let model = {
        let span = tracing::info_span!("merge");
        let _guard = span.enter();
        let mut model: UnresolvedModel = Default::default();

        // process any schemas specified in the configuration first
        if let Some(schemas) = &config.schemas {
            for xsd in schemas {
                // let xsd = parent_dir.join(schema);
                tracing::info!("merging {}", xsd.display());
                model.merge_xsd(&xsd);
            }
        }

        // process any manually specified schemas
        for xsd in options.inputs.iter() {
            tracing::info!("merging {}", xsd.display());
            model.merge_xsd(xsd);
        }
        model
    };

    tracing::info!("Merge complete.  Resolving model.\n");

    let model = model.resolve(config);

    tracing::info!("Resolve complete.  Writing model.\n");

    create_main_output_dir(&options.output, options.remove_dir)?;

    let span = tracing::info_span!("generate");
    let _guard = span.enter();
    gen::write_model(&options.output, model)?;

    Ok(())
}

fn create_main_output_dir(path: &Path, delete_dir: bool) -> Result<(), FatalError> {
    if path.exists() {
        if path.is_file() {
            return Err(format!(
                "Output must be a directory, but the supplied path is a file: {:?}",
                path
            )
            .into());
        }

        if path.is_dir() {
            if delete_dir {
                std::fs::remove_dir_all(path)?;
            } else {
                return Err(format!("Cannot write into existing directory {:?}. Delete the directory or use the -r flag to remove it", path).into());
            }
        }
    }

    std::fs::create_dir(path)?;

    Ok(())
}
